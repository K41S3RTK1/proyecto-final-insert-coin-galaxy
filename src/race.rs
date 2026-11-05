//! Fifth world: continuous steering, ring economy, spin dash and a physical loop.
use crate::{feedback::Cue, input::Input, math::Vec3};
use std::f32::consts::TAU;
pub const GOAL: f32 = 1050.;
pub const LOOP_START: f32 = 300.;
pub const LOOP_RADIUS: f32 = 12.;
pub const LOOP_LENGTH: f32 = TAU * LOOP_RADIUS;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hero {
    Sonic,
    Tails,
    Knuckles,
}
impl Hero {
    pub const ALL: [Self; 3] = [Self::Sonic, Self::Tails, Self::Knuckles];
    pub fn index(self) -> usize {
        match self {
            Self::Sonic => 0,
            Self::Tails => 1,
            Self::Knuckles => 2,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Sonic => "SONIC",
            Self::Tails => "TAILS",
            Self::Knuckles => "KNUCKLES",
        }
    }
    pub fn color(self) -> [u8; 3] {
        match self {
            Self::Sonic => [32, 95, 247],
            Self::Tails => [255, 176, 26],
            Self::Knuckles => [237, 45, 56],
        }
    }
    pub fn skill(self) -> &'static str {
        match self {
            Self::Sonic => "DASH / ATAQUE DIRIGIDO",
            Self::Tails => "VUELO CON DOS COLAS",
            Self::Knuckles => "PLANEO Y GOLPE FUERTE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Choose,
    Countdown,
    Running,
    Finished,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Ring,
    Spikes,
    Robot,
    Crate,
    Spring,
    Boost,
}
#[derive(Clone, Debug)]
pub struct Item {
    pub s: f32,
    pub x: f32,
    pub kind: Kind,
    pub taken: bool,
}
#[derive(Clone, Copy)]
pub struct Frame {
    pub p: Vec3,
    pub up: Vec3,
    pub pitch: f32,
}
pub fn frame(s: f32) -> Frame {
    let in_loop = (LOOP_START..LOOP_START + LOOP_LENGTH).contains(&s);
    let route = if in_loop {
        LOOP_START
    } else if s >= LOOP_START + LOOP_LENGTH {
        s - LOOP_LENGTH
    } else {
        s
    };
    let x = (route * 0.018).sin() * 7. + (route * 0.006).sin() * 8.;
    if in_loop {
        let a = (s - LOOP_START) / LOOP_RADIUS;
        Frame {
            p: Vec3::new(
                x,
                LOOP_RADIUS * (1. - a.cos()),
                -LOOP_START - LOOP_RADIUS * a.sin(),
            ),
            up: Vec3::new(0., a.cos(), a.sin()),
            pitch: -a,
        }
    } else {
        Frame {
            p: Vec3::new(x, 0., -route),
            up: Vec3::new(0., 1., 0.),
            pitch: 0.,
        }
    }
}
pub fn gap(s: f32) -> bool {
    [(160., 175.), (540., 558.), (790., 807.)]
        .iter()
        .any(|&(a, b)| s > a && s < b)
}
pub struct Race {
    pub hero: Hero,
    pub phase: Phase,
    pub countdown: f32,
    pub time: f32,
    pub distance: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub rings: u32,
    pub energy: f32,
    pub lives: i32,
    pub charge: f32,
    pub rolling: f32,
    pub ability: f32,
    pub cooldown: f32,
    pub invulnerable: f32,
    pub checkpoint: f32,
    pub rivals: [f32; 2],
    pub rank: usize,
    pub items: Vec<Item>,
    pub notice: &'static str,
    pub notice_left: f32,
    pub boosting: bool,
    pub air_time: f32,
    selector_latched: bool,
    was_braking: bool,
    homing_x: Option<f32>,
}
impl Race {
    pub fn new() -> Self {
        let mut items = Vec::new();
        for k in 0..125 {
            let s = 18. + k as f32 * 8.;
            items.push(Item {
                s,
                x: if (LOOP_START - 10. ..LOOP_START + LOOP_LENGTH + 10.).contains(&s) {
                    0.
                } else {
                    (k as f32 * 0.35).sin() * 3.2
                },
                kind: Kind::Ring,
                taken: false,
            });
        }
        for (s, x, kind) in [
            (74., -2.5, Kind::Robot),
            (116., 2.5, Kind::Crate),
            (140., 0., Kind::Spring),
            (210., -2., Kind::Spikes),
            (252., 0., Kind::Boost),
            (415., 2., Kind::Robot),
            (465., -2.5, Kind::Crate),
            (515., 0., Kind::Spring),
            (620., 0., Kind::Boost),
            (700., 2.5, Kind::Robot),
            (760., 0., Kind::Spring),
            (850., -2., Kind::Spikes),
            (920., 0., Kind::Boost),
            (976., 1.5, Kind::Robot),
        ] {
            items.push(Item {
                s,
                x,
                kind,
                taken: false,
            });
        }
        Self {
            hero: Hero::Sonic,
            phase: Phase::Choose,
            countdown: 3.,
            time: 0.,
            distance: 0.,
            speed: 0.,
            x: 0.,
            y: 0.,
            vy: 0.,
            rings: 0,
            energy: 100.,
            lives: 3,
            charge: 0.,
            rolling: 0.,
            ability: 0.,
            cooldown: 0.,
            invulnerable: 0.,
            checkpoint: 0.,
            rivals: [0., 0.],
            rank: 1,
            items,
            notice: "ELIGE TU PERSONAJE",
            notice_left: 0.,
            boosting: false,
            air_time: 0.,
            selector_latched: false,
            was_braking: false,
            homing_x: None,
        }
    }
    pub fn position(&self) -> Vec3 {
        let f = frame(self.distance);
        f.p + Vec3::new(self.x, 0., 0.) + f.up * self.y
    }
    pub fn in_loop(&self) -> bool {
        (LOOP_START..LOOP_START + LOOP_LENGTH).contains(&self.distance)
    }
    fn notice(&mut self, text: &'static str) {
        self.notice = text;
        self.notice_left = 1.5;
    }
    fn hurt(&mut self, events: &mut Vec<Cue>) {
        if self.invulnerable > 0. {
            return;
        }
        if self.rings > 0 {
            self.rings = 0;
            self.notice("ANILLOS PERDIDOS");
        } else {
            self.lives -= 1;
            self.notice("IMPACTO / PIERDES UNA VIDA");
        }
        self.invulnerable = 1.8;
        self.speed *= 0.45;
        events.push(Cue::Hurt);
    }
    fn fall(&mut self, events: &mut Vec<Cue>) {
        self.lives -= 1;
        self.rings = 0;
        self.distance = self.checkpoint;
        self.x = 0.;
        self.y = 0.;
        self.vy = 0.;
        self.speed = 8.;
        self.invulnerable = 2.;
        self.ability = 0.;
        self.rolling = 0.;
        self.notice("REGRESO AL CHECKPOINT");
        events.push(Cue::Hurt);
    }
    pub fn tick(&mut self, i: Input, dt: f32, events: &mut Vec<Cue>) {
        if self.phase == Phase::Choose {
            let axis = i.strafe + i.yaw;
            if axis.abs() < 0.25 {
                self.selector_latched = false;
            } else if !self.selector_latched {
                self.hero = Hero::ALL
                    [(self.hero.index() as i32 + if axis > 0. { 1 } else { 2 }) as usize % 3];
                self.selector_latched = true;
                events.push(Cue::Coin);
            }
            if i.accept {
                self.phase = Phase::Countdown;
                self.countdown = 3.;
                events.push(Cue::Count);
            }
            return;
        }
        if self.phase == Phase::Countdown {
            let old = self.countdown.ceil();
            self.countdown = (self.countdown - dt).max(0.);
            if self.countdown.ceil() != old {
                events.push(Cue::Count);
            }
            if self.countdown == 0. {
                self.phase = Phase::Running;
                self.notice("GO! SIGUE LOS ANILLOS");
                events.push(Cue::Portal);
            }
            return;
        }
        if self.phase == Phase::Finished || self.lives <= 0 {
            return;
        }
        self.time += dt;
        self.invulnerable = (self.invulnerable - dt).max(0.);
        self.notice_left = (self.notice_left - dt).max(0.);
        self.rolling = (self.rolling - dt).max(0.);
        self.ability = (self.ability - dt).max(0.);
        self.cooldown = (self.cooldown - dt).max(0.);
        self.x = (self.x + (i.strafe + i.yaw).clamp(-1., 1.) * 9. * dt).clamp(-5.25, 5.25);
        let on_loop = self.in_loop();
        let grounded = self.y <= 0.001 && self.vy <= 0. && !gap(self.distance);
        if i.brake && grounded && !on_loop {
            self.charge = (self.charge + dt * 0.95).min(1.);
            self.speed = (self.speed - 55. * dt).max(0.);
        } else if self.was_braking && self.charge > 0.12 {
            self.speed = 36. + self.charge * 24.;
            self.rolling = 1.5;
            self.charge = 0.;
            events.push(Cue::Shot);
            self.notice("SPIN DASH!");
        } else {
            self.charge = 0.;
        }
        self.was_braking = i.brake && grounded && !on_loop;
        if i.jump && grounded && !on_loop {
            self.vy = 12.;
            self.y = 0.02;
            events.push(Cue::Jump);
        }
        if i.race_ability && self.cooldown <= 0. {
            self.cooldown = 5.;
            events.push(Cue::Dodge);
            match self.hero {
                Hero::Sonic => {
                    self.ability = 0.65;
                    self.rolling = 0.85;
                    self.speed = 56.;
                    if let Some(target) = self
                        .items
                        .iter()
                        .filter(|v| {
                            !v.taken
                                && v.kind == Kind::Robot
                                && v.s > self.distance
                                && v.s - self.distance < 20.
                        })
                        .min_by(|a, b| a.s.total_cmp(&b.s))
                    {
                        self.homing_x = Some(target.x);
                        self.vy = 3.;
                        self.y = self.y.max(0.5);
                    }
                    self.notice("SONIC / HOMING DASH");
                }
                Hero::Tails => {
                    if !on_loop {
                        self.ability = 2.4;
                        self.vy = 5.;
                    }
                    self.notice("TAILS / VUELO");
                }
                Hero::Knuckles => {
                    if !on_loop {
                        self.ability = 2.8;
                        self.vy = 7.;
                    }
                    self.notice("KNUCKLES / PLANEO");
                }
            }
        }
        if self.ability > 0. && self.hero == Hero::Sonic {
            if let Some(target) = self.homing_x {
                self.x += (target - self.x) * (1. - (-18. * dt).exp());
            }
        } else {
            self.homing_x = None;
        }
        self.boosting = i.race_boost && self.energy > 0. && !i.brake;
        let maximum = match self.hero {
            Hero::Sonic => 32.,
            Hero::Tails => 29.,
            Hero::Knuckles => 30.,
        };
        let target = if self.boosting {
            maximum + 15.
        } else {
            maximum - 3. + i.forward.clamp(-1., 1.) * 3.
        };
        if !self.was_braking {
            let accel = if self.speed > target {
                if self.rolling > 0. || self.ability > 0. {
                    5.
                } else {
                    15.
                }
            } else {
                18.
            };
            self.speed += (target - self.speed).clamp(-accel * dt, accel * dt);
        }
        self.energy =
            (self.energy + if self.boosting { -25. * dt } else { 6. * dt }).clamp(0., 100.);
        let old = self.distance;
        self.distance += self.speed * dt;
        if on_loop {
            self.y = 0.;
            self.vy = 0.;
        } else {
            let flying = self.hero == Hero::Tails && self.ability > 0.;
            let gliding = self.hero == Hero::Knuckles && self.ability > 0.;
            self.vy -= if flying { 2. } else { 28. } * dt;
            if flying {
                self.vy = (self.vy + 8. * dt).min(3.);
            }
            if gliding && self.vy < -1.5 {
                self.vy = -1.5;
            }
            self.y += self.vy * dt;
            if !gap(self.distance) && self.y <= 0. {
                self.y = 0.;
                self.vy = 0.;
            }
            if self.y > 8. {
                self.y = 8.;
                self.vy = self.vy.min(0.);
            }
        }
        self.air_time = if self.y > 0.05 {
            self.air_time + dt
        } else {
            0.
        };
        if self.y < -5. {
            self.fall(events);
        }
        // Swept longitudinal intersections avoid tunnelling through items at boost speed.
        for k in 0..self.items.len() {
            let item = &self.items[k];
            if item.taken
                || item.s < old - 0.7
                || item.s > self.distance + 0.7
                || (item.x - self.x).abs() > if item.kind == Kind::Ring { 0.9 } else { 1.2 }
            {
                continue;
            }
            let kind = item.kind;
            match kind {
                Kind::Ring => {
                    if self.y < 2.2 {
                        self.rings += 1;
                        self.energy = (self.energy + 3.).min(100.);
                        self.items[k].taken = true;
                        events.push(Cue::Coin);
                    }
                }
                Kind::Boost => {
                    if self.y < 0.5 {
                        self.speed = 54.;
                        self.rolling = 0.9;
                        self.items[k].taken = true;
                        self.notice("ACELERADOR!");
                        events.push(Cue::Shot);
                    }
                }
                Kind::Spring => {
                    if self.y < 1. {
                        self.vy = 16.;
                        self.y = 0.1;
                        self.items[k].taken = true;
                        self.notice("RESORTE / SALTO ALTO");
                        events.push(Cue::Jump);
                    }
                }
                Kind::Crate | Kind::Robot => {
                    if self.y < 1.7 {
                        if self.rolling > 0.
                            || self.boosting
                            || self.hero == Hero::Knuckles && self.ability > 0.
                        {
                            self.items[k].taken = true;
                            self.rings += 3;
                            events.push(Cue::Hit);
                            self.notice("ENEMIGO DESTRUIDO +3");
                        } else {
                            self.items[k].taken = true;
                            self.hurt(events);
                        }
                    }
                }
                Kind::Spikes => {
                    if self.y < 1.2 {
                        self.items[k].taken = true;
                        self.hurt(events);
                    }
                }
            }
        }
        for cp in [240., 480., 720., 900.] {
            if self.distance >= cp && self.checkpoint < cp {
                self.checkpoint = cp;
                self.notice("CHECKPOINT");
                events.push(Cue::Coin);
            }
        }
        for (k, opponent) in self.rivals.iter_mut().enumerate() {
            *opponent = (*opponent
                + (22.4 + k as f32 * 0.6 + (self.time * 0.5 + k as f32).sin() * 1.8) * dt)
                .min(GOAL);
        }
        self.rank = 1 + self.rivals.iter().filter(|s| **s > self.distance).count();
        if self.distance >= GOAL {
            self.distance = GOAL;
            self.phase = Phase::Finished;
            self.rank = 1 + self.rivals.iter().filter(|s| **s >= GOAL).count();
        }
    }
}
pub(crate) fn route_input(r: &Race) -> Input {
    let hazard = r
        .items
        .iter()
        .filter(|v| {
            !v.taken
                && matches!(v.kind, Kind::Spikes | Kind::Crate | Kind::Robot)
                && v.s > r.distance
                && v.s - r.distance < 12.
        })
        .min_by(|a, b| a.s.total_cmp(&b.s));
    let target = if let Some(h) = hazard {
        if h.x >= 0. { -3. } else { 3. }
    } else {
        r.items
            .iter()
            .filter(|v| !v.taken && v.kind == Kind::Ring && v.s > r.distance)
            .min_by(|a, b| a.s.total_cmp(&b.s))
            .map_or(0., |v| v.x)
    };
    Input {
        accept: r.phase == Phase::Choose,
        strafe: ((target - r.x) * 2.).clamp(-1., 1.),
        jump: r.y < 0.01 && gap(r.distance + 9.),
        race_boost: r.energy > 15. && !gap(r.distance + 20.),
        ..Input::default()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_latches_and_countdown_does_not_move() {
        let mut r = Race::new();
        let mut e = vec![];
        for _ in 0..100 {
            r.tick(
                Input {
                    strafe: 1.,
                    ..Input::default()
                },
                1. / 120.,
                &mut e,
            );
        }
        assert_eq!(r.hero, Hero::Tails);
        r.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
            &mut e,
        );
        assert_eq!(r.phase, Phase::Countdown);
        assert_eq!(r.distance, 0.);
    }
    #[test]
    fn all_heroes_can_complete_race_through_real_inputs() {
        for hero in Hero::ALL {
            let mut r = Race::new();
            r.hero = hero;
            let mut e = vec![];
            for _ in 0..120 * 100 {
                let i = route_input(&r);
                r.tick(i, 1. / 120., &mut e);
                e.clear();
                if r.phase == Phase::Finished || r.lives <= 0 {
                    break;
                }
            }
            assert_eq!(r.phase, Phase::Finished, "{hero:?}: {}", r.distance);
            assert_eq!(r.rank, 1);
            assert!(r.lives > 0);
            assert!(r.rings > 0);
        }
    }
    #[test]
    fn spin_charge_requires_release_and_abilities_are_distinct() {
        for hero in Hero::ALL {
            let mut r = Race::new();
            r.phase = Phase::Running;
            r.hero = hero;
            let mut e = vec![];
            for _ in 0..120 {
                r.tick(
                    Input {
                        brake: true,
                        ..Input::default()
                    },
                    1. / 120.,
                    &mut e,
                );
            }
            assert!(r.charge > 0.8);
            assert_eq!(r.distance, 0.);
            r.tick(Input::default(), 1. / 120., &mut e);
            assert!(r.speed > 50. && r.rolling > 0.);
            r.tick(
                Input {
                    race_ability: true,
                    ..Input::default()
                },
                1. / 120.,
                &mut e,
            );
            assert!(r.ability > 0.);
            assert!(r.cooldown > 0.);
            if hero != Hero::Sonic {
                assert!(r.vy > 0.);
            }
        }
    }
    #[test]
    fn loop_is_continuous_and_completes_a_full_rotation() {
        for s in [LOOP_START, LOOP_START + LOOP_LENGTH] {
            assert!((frame(s - 0.001).p - frame(s + 0.001).p).length() < 0.01);
        }
        let f = frame(LOOP_START + LOOP_LENGTH / 2.);
        assert!(f.up.y < -0.99);
        assert!((f.p.y - 24.).abs() < 0.01);
    }
    #[test]
    fn rings_protect_once_and_fall_uses_checkpoint() {
        let mut r = Race::new();
        let mut e = vec![];
        r.rings = 12;
        r.hurt(&mut e);
        assert_eq!((r.rings, r.lives), (0, 3));
        r.invulnerable = 0.;
        r.hurt(&mut e);
        assert_eq!(r.lives, 2);
        r.checkpoint = 240.;
        r.fall(&mut e);
        assert_eq!(r.distance, 240.);
        assert_eq!(r.lives, 1);
    }
}
