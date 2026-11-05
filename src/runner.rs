//! Three-lane runner. Rows always leave a reachable open lane; world positions
//! are distances along the route, so collisions don't depend on render FPS.
use crate::{feedback::Cue, input::Input};
pub const GOAL: f32 = 900.;
pub const REQUIRED_COINS: u32 = 20;
pub const PLAYER_Z: f32 = 6.;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Train,
    Barrier,
    Beam,
    Coin,
}
#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub lane: i32,
    pub at: f32,
    pub kind: Kind,
    pub resolved: bool,
}
pub struct Runner {
    pub distance: f32,
    pub speed: f32,
    pub lane: i32,
    pub x: f32,
    pub jump: f32,
    pub slide: f32,
    pub health: i32,
    pub coins: u32,
    pub invulnerable: f32,
    pub flash: f32,
    pub coin_flash: f32,
    pub endless: bool,
    pub items: Vec<Item>,
    next_row: u32,
    horizontal_latched: i32,
}
impl Runner {
    pub fn new() -> Self {
        let mut s = Self {
            distance: 0.,
            speed: 12.,
            lane: 0,
            x: 0.,
            jump: 0.,
            slide: 0.,
            health: 3,
            coins: 0,
            invulnerable: 0.,
            flash: 0.,
            coin_flash: 0.,
            endless: false,
            items: vec![],
            next_row: 0,
            horizontal_latched: 0,
        };
        s.populate();
        s
    }
    pub fn height(&self) -> f32 {
        if self.jump <= 0. {
            0.
        } else {
            (self.jump / 0.95 * std::f32::consts::PI).sin() * 2.4
        }
    }
    pub fn score(&self) -> u32 {
        self.distance.floor() as u32 + self.coins * 25
    }
    pub fn completed(&self) -> bool {
        self.distance >= GOAL && self.coins >= REQUIRED_COINS
    }
    fn populate(&mut self) {
        while 42. + self.next_row as f32 * 26. < self.distance + 120. {
            let row = self.next_row;
            let at = 42. + row as f32 * 26.;
            // First rows teach jump, slide, then solid trains. Later rows combine.
            let safe = match row % 6 {
                0 | 5 => -1,
                1 | 4 => 1,
                _ => 0,
            };
            for lane in -1..=1 {
                if lane != safe {
                    let kind = match (row + (lane + 1) as u32) % 3 {
                        0 => Kind::Barrier,
                        1 => Kind::Beam,
                        _ => Kind::Train,
                    };
                    self.items.push(Item {
                        lane,
                        at,
                        kind,
                        resolved: false,
                    });
                }
            }
            for spacing in [0., 4., 8., 12.] {
                self.items.push(Item {
                    lane: safe,
                    at: at + spacing,
                    kind: Kind::Coin,
                    resolved: false,
                });
            }
            self.next_row += 1;
        }
    }
    pub fn tick(&mut self, i: Input, dt: f32, events: &mut Vec<Cue>) {
        self.invulnerable = (self.invulnerable - dt).max(0.);
        self.flash = (self.flash - dt).max(0.);
        self.coin_flash = (self.coin_flash - dt).max(0.);
        self.jump = (self.jump - dt).max(0.);
        self.slide = (self.slide - dt).max(0.);
        let axis = (i.strafe + i.yaw).clamp(-1., 1.);
        let direction = if axis > 0.45 {
            1
        } else if axis < -0.45 {
            -1
        } else {
            0
        };
        if direction != 0 && direction != self.horizontal_latched {
            self.lane = (self.lane + direction).clamp(-1, 1);
        }
        self.horizontal_latched = direction;
        self.x += (self.lane as f32 * 3. - self.x) * (1. - (-18. * dt).exp());
        if i.jump && self.jump <= 0. && self.slide <= 0. {
            self.jump = 0.95;
            events.push(Cue::Jump);
        }
        if i.slide && self.slide <= 0. && self.jump <= 0. {
            self.slide = 0.80;
            events.push(Cue::Dodge);
        }
        let previous = self.distance;
        self.speed = (12. + self.distance * 0.012).min(23.);
        self.distance += self.speed * dt;
        let height = self.height();
        for item in &mut self.items {
            if item.resolved {
                continue;
            }
            let near_lane = (self.x - item.lane as f32 * 3.).abs()
                < if item.kind == Kind::Coin { 1.15 } else { 1.0 };
            let ahead = item.at - self.distance;
            let behind = if item.kind == Kind::Train { -6. } else { -0.65 };
            let crosses = item.at >= previous - 0.65 && item.at <= self.distance + 0.65;
            if item.kind == Kind::Coin {
                if crosses && near_lane {
                    self.coins += 1;
                    self.coin_flash = 0.2;
                    item.resolved = true;
                    events.push(Cue::Coin);
                }
            } else if ahead >= behind && (ahead <= 0.65 || crosses) && near_lane {
                let defended = match item.kind {
                    Kind::Barrier => height > 1.15,
                    Kind::Beam => self.slide > 0.,
                    _ => false,
                };
                if !defended && self.invulnerable <= 0. {
                    self.health -= 1;
                    self.invulnerable = 1.35;
                    self.flash = 0.3;
                    item.resolved = true;
                    events.push(Cue::Hurt);
                }
            }
            if ahead < behind {
                item.resolved = true;
            }
        }
        self.items.retain(|item| item.at > self.distance - 12.);
        self.populate();
    }
}
pub(crate) fn route_input(r: &Runner, release: bool) -> Input {
    if release {
        return Input::default();
    }
    let next = r
        .items
        .iter()
        .filter(|v| v.kind != Kind::Coin && v.at > r.distance + 7.)
        .min_by(|a, b| a.at.total_cmp(&b.at));
    let Some(next) = next else {
        return Input::default();
    };
    if next.at - r.distance > 18. {
        return Input::default();
    }
    let safe = (-1..=1)
        .find(|lane| {
            !r.items
                .iter()
                .any(|v| v.kind != Kind::Coin && v.at == next.at && v.lane == *lane)
        })
        .unwrap();
    Input {
        strafe: (safe - r.lane).signum() as f32,
        ..Input::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn isolated(kind: Kind, at: f32) -> Runner {
        let mut r = Runner::new();
        r.items = vec![Item {
            kind,
            at,
            lane: 0,
            resolved: false,
        }];
        r.next_row = 500;
        r
    }
    #[test]
    fn jump_slide_and_solid_train_collisions() {
        for (kind, input, expected) in [
            (
                Kind::Barrier,
                Input {
                    jump: true,
                    ..Default::default()
                },
                3,
            ),
            (
                Kind::Beam,
                Input {
                    slide: true,
                    ..Default::default()
                },
                3,
            ),
            (
                Kind::Train,
                Input {
                    jump: true,
                    ..Default::default()
                },
                2,
            ),
            (Kind::Barrier, Input::default(), 2),
            (
                Kind::Beam,
                Input {
                    jump: true,
                    ..Default::default()
                },
                2,
            ),
        ] {
            let mut r = isolated(kind, 4.);
            let mut i = input;
            for _ in 0..90 {
                r.tick(i, 1. / 120., &mut vec![]);
                i.clear_edges();
            }
            assert_eq!(r.health, expected, "{kind:?}");
        }
    }
    #[test]
    fn held_stick_moves_one_lane_and_neutral_rearms() {
        let mut r = Runner::new();
        for _ in 0..30 {
            r.tick(
                Input {
                    strafe: -1.,
                    ..Default::default()
                },
                0.01,
                &mut vec![],
            );
        }
        assert_eq!(r.lane, -1);
        r.tick(
            Input {
                strafe: 1.,
                ..Default::default()
            },
            0.01,
            &mut vec![],
        );
        assert_eq!(r.lane, 0);
        r.tick(Input::default(), 0.01, &mut vec![]);
        r.tick(
            Input {
                strafe: 1.,
                ..Default::default()
            },
            0.01,
            &mut vec![],
        );
        assert_eq!(r.lane, 1);
    }
    #[test]
    fn coin_is_collected_once_and_off_lane_misses() {
        let mut r = isolated(Kind::Coin, 2.);
        for _ in 0..120 {
            r.tick(Input::default(), 1. / 120., &mut vec![]);
        }
        assert_eq!(r.coins, 1);
        let mut r = isolated(Kind::Coin, 5.);
        for _ in 0..120 {
            r.tick(
                Input {
                    strafe: -1.,
                    ..Default::default()
                },
                1. / 120.,
                &mut vec![],
            );
        }
        assert_eq!(r.coins, 0);
    }
    #[test]
    fn generated_rows_always_leave_a_lane_and_memory_is_bounded() {
        let mut r = Runner::new();
        for row in 0..500 {
            r.distance = row as f32 * 26.;
            r.tick(Input::default(), 0., &mut vec![]);
            assert!(r.items.len() < 40);
            let mut rows = std::collections::BTreeMap::<i32, usize>::new();
            for item in r.items.iter().filter(|v| v.kind != Kind::Coin) {
                *rows.entry(item.at as i32).or_default() += 1;
            }
            assert!(rows.values().all(|n| *n == 2));
        }
    }
}
