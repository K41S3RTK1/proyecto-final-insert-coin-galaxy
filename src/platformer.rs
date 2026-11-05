//! Side-scrolling platform physics: independent axes, swept landing/head hits,
//! coyote time, jump buffer, power-up, enemies, checkpoint and flag finish.
use crate::{feedback::Cue, input::Input};
pub const TILE: f32 = 1.2;
pub const FLAG_X: f32 = 198. * TILE;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockKind {
    Brick,
    Question,
    Used,
    Stair,
    Pipe,
    Ground,
}
#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub kind: BlockKind,
    pub broken: bool,
    pub bump: f32,
}
#[derive(Clone, Copy)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub left: f32,
    pub right: f32,
    pub vx: f32,
    pub alive: bool,
    pub squash: f32,
}
#[derive(Clone, Copy)]
pub struct Coin {
    pub x: f32,
    pub y: f32,
    pub taken: bool,
}
#[derive(Clone, Copy)]
pub struct Mushroom {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub vx: f32,
    pub active: bool,
}
pub struct Platformer {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub grounded: bool,
    pub facing: f32,
    pub lives: i32,
    pub big: bool,
    pub coins: u32,
    pub score: u32,
    pub time: f32,
    pub invulnerable: f32,
    pub flash: f32,
    pub checkpoint: f32,
    pub camera: f32,
    pub finished: bool,
    pub blocks: Vec<Block>,
    pub enemies: Vec<Enemy>,
    pub pickups: Vec<Coin>,
    pub mushroom: Option<Mushroom>,
    coyote: f32,
    jump_buffer: f32,
    power_given: bool,
}
impl Platformer {
    pub fn new() -> Self {
        let mut blocks = vec![];
        let mut add = |x: f32, y: f32, w: f32, h: f32, kind| {
            blocks.push(Block {
                x: x * TILE,
                y: y * TILE,
                w: w * TILE,
                h: h * TILE,
                kind,
                broken: false,
                bump: 0.,
            })
        };
        for (a, b) in [(0., 69.), (71., 86.), (89., 153.), (155., 213.)] {
            add(a, -3., b - a, 3., BlockKind::Ground);
        }
        for (x, h) in [
            (28., 2.),
            (38., 3.),
            (46., 4.),
            (57., 4.),
            (163., 2.),
            (179., 2.),
        ] {
            add(x, 0., 2., h, BlockKind::Pipe);
        }
        for (x, y, q) in [
            (16., 4., true),
            (20., 4., false),
            (21., 4., true),
            (22., 4., false),
            (23., 4., true),
            (24., 4., false),
            (22., 8., true),
            (77., 4., false),
            (78., 4., true),
            (79., 4., false),
            (80., 8., false),
            (81., 8., false),
            (82., 8., false),
            (83., 8., false),
            (84., 8., false),
            (85., 8., false),
            (86., 8., false),
            (87., 8., false),
            (91., 8., false),
            (92., 8., false),
            (93., 8., false),
            (94., 8., true),
            (94., 4., false),
            (100., 4., false),
            (101., 4., false),
            (106., 4., true),
            (109., 4., true),
            (112., 4., true),
            (109., 8., true),
            (118., 4., false),
            (121., 8., false),
            (122., 8., false),
            (123., 8., false),
            (128., 8., false),
            (129., 8., true),
            (130., 8., true),
            (131., 8., false),
            (129., 4., false),
            (130., 4., false),
            (168., 4., false),
            (169., 4., false),
            (170., 4., true),
            (171., 4., false),
        ] {
            add(
                x,
                y,
                1.,
                1.,
                if q {
                    BlockKind::Question
                } else {
                    BlockKind::Brick
                },
            );
        }
        for start in [134., 148.] {
            for k in 0..4 {
                add(start + k as f32, 0., 1., (k + 1) as f32, BlockKind::Stair);
            }
        }
        for k in 0..4 {
            add(140. + k as f32, 0., 1., (4 - k) as f32, BlockKind::Stair);
        }
        for k in 0..4 {
            add(155. + k as f32, 0., 1., (4 - k) as f32, BlockKind::Stair);
        }
        for k in 0..8 {
            add(181. + k as f32, 0., 1., (k + 1) as f32, BlockKind::Stair);
        }
        let enemies = [
            22., 40., 51., 53., 80., 82., 97., 99., 114., 116., 124., 126., 174., 176.,
        ]
        .iter()
        .map(|x| Enemy {
            x: x * TILE,
            y: 0.,
            left: (x - 1.5) * TILE,
            right: (x + 1.5) * TILE,
            vx: -1.0,
            alive: true,
            squash: 0.,
        })
        .collect();
        let pickups = [
            (18., 6.),
            (33., 4.),
            (63., 3.),
            (65., 3.),
            (73., 3.),
            (103., 3.),
            (120., 6.),
            (145., 4.),
            (160., 3.),
            (173., 4.),
            (191., 6.),
            (194., 4.),
        ]
        .iter()
        .map(|(x, y)| Coin {
            x: x * TILE,
            y: y * TILE,
            taken: false,
        })
        .collect();
        Self {
            x: 3.,
            y: 0.,
            vx: 0.,
            vy: 0.,
            grounded: true,
            facing: 1.,
            lives: 3,
            big: false,
            coins: 0,
            score: 0,
            time: 0.,
            invulnerable: 0.,
            flash: 0.,
            checkpoint: 3.,
            camera: 11.,
            finished: false,
            blocks,
            enemies,
            pickups,
            mushroom: None,
            coyote: 0.1,
            jump_buffer: 0.,
            power_given: false,
        }
    }
    pub fn height(&self) -> f32 {
        if self.big { 2.05 } else { 1.6 }
    }
    fn damage(&mut self, fall: bool, events: &mut Vec<Cue>) {
        if self.invulnerable > 0. && !fall {
            return;
        }
        events.push(Cue::Hurt);
        self.flash = 0.3;
        if self.big && !fall {
            self.big = false;
            self.invulnerable = 1.8;
            self.vx = -self.facing * 3.;
            return;
        }
        self.lives -= 1;
        self.big = false;
        self.invulnerable = 2.;
        self.x = self.checkpoint;
        self.y = 0.;
        self.vx = 0.;
        self.vy = 0.;
        self.grounded = true;
    }
    pub fn tick(&mut self, i: Input, dt: f32, events: &mut Vec<Cue>) {
        if self.finished || self.lives <= 0 {
            return;
        }
        self.time += dt;
        self.invulnerable = (self.invulnerable - dt).max(0.);
        self.flash = (self.flash - dt).max(0.);
        for b in &mut self.blocks {
            b.bump = (b.bump - dt).max(0.);
        }
        if self.grounded {
            self.coyote = 0.10;
        } else {
            self.coyote = (self.coyote - dt).max(0.);
        }
        self.jump_buffer = if i.jump {
            0.12
        } else {
            (self.jump_buffer - dt).max(0.)
        };
        let axis = (i.strafe + i.yaw).clamp(-1., 1.);
        let target = axis * if i.boost || i.fire { 9.5 } else { 6.5 };
        self.vx +=
            (target - self.vx) * (1. - (-if axis.abs() > 0.1 { 15. } else { 20. } * dt).exp());
        if axis.abs() > 0.1 {
            self.facing = axis.signum();
        }
        if self.jump_buffer > 0. && self.coyote > 0. {
            self.vy = 18.;
            self.grounded = false;
            self.coyote = 0.;
            self.jump_buffer = 0.;
            events.push(Cue::Jump);
        }
        let h = self.height();
        self.x = (self.x + self.vx * dt).clamp(0.5, FLAG_X + 5.);
        for b in &self.blocks {
            if b.broken {
                continue;
            }
            if self.y + h > b.y + 0.03
                && self.y < b.y + b.h - 0.03
                && self.x + 0.34 > b.x
                && self.x - 0.34 < b.x + b.w
            {
                if self.vx > 0. {
                    self.x = b.x - 0.34;
                } else if self.vx < 0. {
                    self.x = b.x + b.w + 0.34;
                }
                self.vx = 0.;
            }
        }
        let previous = self.y;
        self.vy -= 30. * dt;
        self.y += self.vy * dt;
        self.grounded = false;
        for b in &mut self.blocks {
            if b.broken || self.x + 0.32 <= b.x || self.x - 0.32 >= b.x + b.w {
                continue;
            }
            if self.vy <= 0. && previous >= b.y + b.h - 0.05 && self.y <= b.y + b.h {
                self.y = b.y + b.h;
                self.vy = 0.;
                self.grounded = true;
            } else if self.vy > 0. && previous + h <= b.y + 0.05 && self.y + h >= b.y {
                self.y = b.y - h;
                self.vy = 0.;
                b.bump = 0.22;
                if b.kind == BlockKind::Question {
                    b.kind = BlockKind::Used;
                    self.score += 100;
                    if !self.power_given {
                        self.power_given = true;
                        self.mushroom = Some(Mushroom {
                            x: b.x + b.w * 0.5,
                            y: b.y + b.h + 0.1,
                            vy: 3.,
                            vx: 1.8,
                            active: true,
                        });
                        events.push(Cue::Phase);
                    } else {
                        self.coins += 1;
                        events.push(Cue::Coin);
                    }
                } else if b.kind == BlockKind::Brick && self.big {
                    b.broken = true;
                    self.score += 50;
                    events.push(Cue::Hit);
                }
            }
        }
        if self.y < -7. {
            self.damage(true, events);
        }
        for coin in &mut self.pickups {
            if !coin.taken
                && (self.x - coin.x).abs() < 0.7
                && (self.y + h * 0.6 - coin.y).abs() < 1.1
            {
                coin.taken = true;
                self.coins += 1;
                self.score += 100;
                events.push(Cue::Coin);
            }
        }
        if let Some(m) = &mut self.mushroom
            && m.active
        {
            let old = m.y;
            m.vy -= 24. * dt;
            m.y += m.vy * dt;
            m.x += m.vx * dt;
            for b in &self.blocks {
                if !b.broken
                    && m.x + 0.3 > b.x
                    && m.x - 0.3 < b.x + b.w
                    && old >= b.y + b.h - 0.05
                    && m.y <= b.y + b.h
                    && m.vy < 0.
                {
                    m.y = b.y + b.h;
                    m.vy = 0.;
                }
            }
            if (m.x - self.x).abs() < 0.8 && (m.y - self.y).abs() < h {
                m.active = false;
                self.big = true;
                self.score += 1000;
                events.push(Cue::Phase);
            }
            if m.y < -8. {
                m.active = false;
            }
        }
        let mut hit = false;
        for e in &mut self.enemies {
            e.squash = (e.squash - dt).max(0.);
            if !e.alive {
                continue;
            }
            e.x += e.vx * dt;
            if e.x < e.left || e.x > e.right {
                e.vx = -e.vx;
            }
            if (self.x - e.x).abs() < 0.70 && self.y < e.y + 0.85 && self.y + h > e.y + 0.05 {
                if self.vy < 0. && previous >= e.y + 0.65 {
                    e.alive = false;
                    e.squash = 0.35;
                    self.vy = 9.;
                    self.score += 200;
                    events.push(Cue::Hit);
                } else {
                    hit = true;
                }
            }
        }
        if hit {
            self.damage(false, events);
        }
        if self.x > 96. * TILE && self.grounded && self.checkpoint < 90. * TILE {
            self.checkpoint = 95. * TILE;
            events.push(Cue::Count);
        }
        if self.x >= FLAG_X {
            self.finished = true;
            self.score += 1000;
        }
        self.camera += (self.x + 5. - self.camera) * (1. - (-7. * dt).exp());
        self.camera = self.camera.clamp(11., FLAG_X - 3.);
    }
}

pub(crate) fn route_input(p: &Platformer) -> Input {
    let wall = p.blocks.iter().any(|b| {
        !b.broken
            && b.x >= p.x - 0.2
            && b.x - p.x < 1.8
            && b.y + b.h > p.y + 0.2
            && b.y < p.y + p.height()
    });
    let enemy = p
        .enemies
        .iter()
        .any(|e| e.alive && e.x > p.x && e.x - p.x < 2.3 && p.y < 1.);
    let ground_ahead = p
        .blocks
        .iter()
        .any(|b| b.kind == BlockKind::Ground && p.x + 2.5 >= b.x && p.x + 2.5 < b.x + b.w);
    let platform_edge = p.y > 0.1
        && !p.blocks.iter().any(|b| {
            !b.broken && (b.y + b.h - p.y).abs() < 0.1 && p.x + 1.2 >= b.x && p.x + 1.2 < b.x + b.w
        });
    Input {
        strafe: 1.,
        jump: p.grounded && (wall || enemy || !ground_ahead || platform_edge),
        ..Default::default()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const DT: f32 = 1. / 120.;
    #[test]
    fn question_block_releases_power_up_once_and_large_mario_breaks_brick() {
        let mut p = Platformer::new();
        let mut ev = vec![];
        p.x = 16.5 * TILE;
        p.tick(
            Input {
                jump: true,
                ..Default::default()
            },
            DT,
            &mut ev,
        );
        for _ in 0..80 {
            p.tick(Input::default(), DT, &mut ev);
        }
        assert_eq!(
            p.blocks.iter().find(|b| b.x == 16. * TILE).unwrap().kind,
            BlockKind::Used
        );
        assert!(p.mushroom.is_some());
        assert_eq!(p.score, 100);
        let m = p.mushroom.unwrap();
        p.x = m.x;
        p.y = m.y;
        p.tick(Input::default(), DT, &mut ev);
        assert!(p.big);
        assert_eq!(p.score, 1100);
        p.x = 20.5 * TILE;
        p.y = 0.;
        p.grounded = true;
        p.vy = 0.;
        p.tick(
            Input {
                jump: true,
                ..Default::default()
            },
            DT,
            &mut ev,
        );
        for _ in 0..80 {
            p.tick(Input::default(), DT, &mut ev);
        }
        assert!(p.blocks.iter().find(|b| b.x == 20. * TILE).unwrap().broken);
    }
    #[test]
    fn stomp_bounces_but_side_hit_shrinks_then_costs_a_life() {
        let mut p = Platformer::new();
        let mut ev = vec![];
        p.blocks.retain(|b| b.kind == BlockKind::Ground);
        p.enemies.truncate(1);
        p.enemies[0].vx = 0.;
        p.x = p.enemies[0].x;
        p.y = 1.8;
        p.vy = -2.;
        p.grounded = false;
        for _ in 0..50 {
            p.tick(Input::default(), DT, &mut ev);
            if !p.enemies[0].alive {
                break;
            }
        }
        assert!(!p.enemies[0].alive);
        assert!(p.vy > 0.);
        assert_eq!(p.lives, 3);
        p.enemies[0].alive = true;
        p.y = 0.;
        p.vy = 0.;
        p.big = true;
        p.tick(Input::default(), DT, &mut ev);
        assert!(!p.big);
        assert_eq!(p.lives, 3);
        p.invulnerable = 0.;
        p.x = p.enemies[0].x;
        p.tick(Input::default(), DT, &mut ev);
        assert_eq!(p.lives, 2);
        assert_eq!(p.x, p.checkpoint);
    }
    #[test]
    fn falling_returns_to_checkpoint_and_buffered_jump_waits_for_landing() {
        let mut p = Platformer::new();
        let mut ev = vec![];
        p.checkpoint = 95. * TILE;
        p.x = 70. * TILE;
        p.y = -7.1;
        p.grounded = false;
        p.tick(Input::default(), DT, &mut ev);
        assert_eq!(p.lives, 2);
        assert_eq!(p.x, p.checkpoint);
        p.x = 3.;
        p.y = 0.15;
        p.vy = -3.;
        p.grounded = false;
        p.coyote = 0.;
        p.tick(
            Input {
                jump: true,
                ..Default::default()
            },
            DT,
            &mut ev,
        );
        for _ in 0..8 {
            p.tick(Input::default(), DT, &mut ev);
        }
        assert!(p.vy > 0., "buffered jump should launch after landing");
    }
    #[test]
    fn full_level_can_be_completed_using_only_movement_and_jump() {
        let mut g = crate::game::Game::new();
        g.active_world = crate::game::WorldId::Mario;
        g.enter();
        for _ in 0..120 * 180 {
            let input = route_input(&g.platformer);
            g.tick(input, DT);
            g.events.clear();
            if matches!(
                g.mode,
                crate::game::Mode::Victory | crate::game::Mode::Defeat
            ) {
                break;
            }
        }
        assert_eq!(
            g.mode,
            crate::game::Mode::Victory,
            "x {} y {} lives {} t {}",
            g.platformer.x,
            g.platformer.y,
            g.platformer.lives,
            g.battle_time
        );
        assert_eq!(g.progress.medals, [false, false, false, true, false]);
        assert!(g.progress.best_ms[3].is_some());
    }
}
