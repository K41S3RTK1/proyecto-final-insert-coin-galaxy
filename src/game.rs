use crate::{
    duel::Duel, feedback::Cue, input::Input, math::Vec3, progress::Progress, runner::Runner,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldId {
    Garden,
    Dojo,
    Runner,
    Mario,
    Race,
}
impl WorldId {
    pub const ALL: [Self; 5] = [
        Self::Garden,
        Self::Dojo,
        Self::Runner,
        Self::Mario,
        Self::Race,
    ];
    pub fn index(self) -> usize {
        match self {
            Self::Garden => 0,
            Self::Dojo => 1,
            Self::Runner => 2,
            Self::Mario => 3,
            Self::Race => 4,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Garden => "THE INKWELL GARDEN",
            Self::Dojo => "NEON DOJO",
            Self::Runner => "BANANA RUSH",
            Self::Mario => "MUSHROOM KINGDOM",
            Self::Race => "SONIC GRAND PRIX",
        }
    }
    pub fn other(self) -> Self {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }
    pub fn color(self) -> [u8; 3] {
        match self {
            Self::Garden => [102, 203, 185],
            Self::Dojo => [222, 112, 177],
            Self::Runner => [255, 209, 68],
            Self::Mario => [247, 99, 73],
            Self::Race => [74, 158, 255],
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    Fan,
    Ring,
    Rain,
}
impl Pattern {
    pub fn name(self) -> &'static str {
        match self {
            Self::Fan => "ABANICO / MUEVETE A UN LADO",
            Self::Ring => "ANILLO / BUSCA UN HUECO",
            Self::Rain => "LLUVIA / EVITA LAS COLUMNAS",
        }
    }
}
#[derive(Clone, Copy)]
pub struct Particle {
    pub p: Vec3,
    pub velocity: Vec3,
    pub life: f32,
    pub color: [u8; 3],
}
pub const INTRO_SECONDS: f32 = 3.4;
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    Title,
    Flight,
    Entering,
    Battle,
    Victory,
    Defeat,
}
#[derive(Clone, Copy)]
pub struct Bullet {
    pub p: Vec3,
    pub v: Vec3,
    pub friendly: bool,
    pub life: f32,
}
pub const BOSS_MAX_HP: i32 = 32;
pub struct Game {
    pub active_world: WorldId,
    pub progress: Progress,
    pub duel: Duel,
    pub runner: Runner,
    pub race: crate::race::Race,
    pub platformer: crate::platformer::Platformer,
    pub vibration_enabled: bool,
    pub vibration_available: bool,
    pub vibration_pulses: u32,
    pub events: Vec<Cue>,
    pub particles: Vec<Particle>,
    pub pattern: Pattern,
    pub phase_two: bool,
    pub phase_notice: f32,
    pub hit_flash: f32,
    pub boss_flash: f32,
    pub warning: bool,
    pub rain_gap: usize,
    pub portal_left: f32,
    portal_origin: Vec3,
    pub audio_muted: bool,
    pub audio_available: bool,
    pub save_error: Option<String>,
    pub arrival: f32,
    return_offset: Vec3,
    return_yaw: f32,
    return_pitch: f32,
    pub time: f32,
    pub mode: Mode,
    pub paused: bool,
    pub ship: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    turn: Vec3,
    pub local_orbit: bool,
    pub player: Vec3,
    pub bullets: Vec<Bullet>,
    pub health: i32,
    pub boss_hp: i32,
    pub battle_time: f32,
    pub invulnerable: f32,
    shot_timer: f32,
    enemy_timer: f32,
    wave: u32,
    pub medal: bool,
    pub camera_eye: Vec3,
    pub camera_target: Vec3,
    pub collisions: u32,
}
impl Game {
    pub fn new() -> Self {
        let ship = Vec3::new(22., 2., 14.);
        Self {
            active_world: WorldId::Garden,
            progress: Progress::default(),
            duel: Duel::new(),
            runner: Runner::new(),
            platformer: crate::platformer::Platformer::new(),
            race: crate::race::Race::new(),
            vibration_enabled: true,
            vibration_available: false,
            vibration_pulses: 0,
            events: vec![],
            particles: vec![],
            pattern: Pattern::Fan,
            phase_two: false,
            phase_notice: 0.,
            hit_flash: 0.,
            boss_flash: 0.,
            warning: false,
            rain_gap: 2,
            portal_left: 0.,
            portal_origin: ship,
            audio_muted: false,
            audio_available: false,
            save_error: None,
            arrival: 0.,
            return_offset: Vec3::new(0., 2., 14.),
            return_yaw: 0.,
            return_pitch: 0.,
            time: 0.,
            mode: Mode::Title,
            paused: false,
            ship,
            velocity: Vec3::default(),
            yaw: 0.,
            pitch: 0.,
            turn: Vec3::default(),
            local_orbit: false,
            player: Vec3::new(-3., 0., 12.),
            bullets: vec![],
            health: 3,
            boss_hp: BOSS_MAX_HP,
            battle_time: 0.,
            invulnerable: 0.,
            shot_timer: 0.,
            enemy_timer: 1.2,
            wave: 0,
            medal: false,
            camera_eye: ship + Vec3::new(0., 4., 9.),
            camera_target: ship + Vec3::new(0., 0., -12.),
            collisions: 0,
        }
    }
    pub fn planet_at(&self, id: WorldId) -> Vec3 {
        let (radius, speed, phase, height) = match id {
            WorldId::Garden => (22., 0.015, 0., 0.),
            WorldId::Dojo => (36., 0.010, -1.9, 2.),
            WorldId::Runner => (46., 0.008, 2.0, -2.),
            WorldId::Mario => (56., 0.006, -0.8, 1.),
            WorldId::Race => (60., 0.007, 3.5, 3.),
        };
        let angle = self.time * speed + phase;
        Vec3::new(angle.cos() * radius, height, angle.sin() * radius)
    }
    pub fn spin_at(&self, id: WorldId) -> f32 {
        self.time
            * match id {
                WorldId::Garden => 0.035,
                WorldId::Dojo => -0.025,
                WorldId::Runner => 0.02,
                WorldId::Mario => -0.018,
                WorldId::Race => 0.023,
            }
    }
    pub fn planet(&self) -> Vec3 {
        self.planet_at(WorldId::Garden)
    }
    pub fn spin(&self) -> f32 {
        self.spin_at(WorldId::Garden)
    }
    pub fn world_at(&self, id: WorldId, p: Vec3) -> Vec3 {
        self.planet_at(id) + p.rotate_y(self.spin_at(id))
    }
    pub fn world(&self, p: Vec3) -> Vec3 {
        self.world_at(WorldId::Garden, p)
    }
    pub fn gate_at(&self, id: WorldId) -> Vec3 {
        self.world_at(id, Vec3::new(0., 1., 10.))
    }
    pub fn gate(&self) -> Vec3 {
        self.gate_at(self.active_world)
    }
    pub fn gate_distance(&self) -> f32 {
        (self.ship - self.gate()).length()
    }
    pub fn can_enter(&self) -> bool {
        self.mode == Mode::Flight && self.gate_distance() < 6.
    }
    pub fn time_limit(&self) -> f32 {
        match self.active_world {
            WorldId::Garden => 75.,
            WorldId::Dojo => 90.,
            WorldId::Runner => f32::INFINITY,
            WorldId::Mario => 300.,
            WorldId::Race => 120.,
        }
    }
    pub fn destination_bearing(&self) -> f32 {
        let v = self.gate() - self.ship;
        let angle = v.x.atan2(-v.z) - self.yaw;
        (angle + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
    }
    fn remember_return(&mut self) {
        self.return_offset = (self.ship - self.planet_at(self.active_world))
            .rotate_y(-self.spin_at(self.active_world));
        self.return_yaw = self.yaw;
        self.return_pitch = self.pitch;
    }
    fn begin_entry(&mut self) {
        self.remember_return();
        self.mode = Mode::Entering;
        self.portal_left = 1.4;
        self.portal_origin = self.ship;
        self.velocity = Vec3::default();
        self.local_orbit = false;
        self.events.push(Cue::Portal);
    }
    pub fn bank(&self) -> f32 {
        -self.turn.x * 0.20
    }
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        )
    }
    pub fn boss(&self) -> Vec3 {
        Vec3::new(
            (self.battle_time * if self.phase_two { 0.9 } else { 0.7 }).sin() * 1.7,
            1.8 + (self.battle_time * 1.1).sin() * 0.5,
            7.,
        )
    }
    pub fn inside(&self) -> bool {
        matches!(self.mode, Mode::Battle | Mode::Victory | Mode::Defeat)
    }
    pub fn enter(&mut self) {
        if self.mode == Mode::Flight || self.mode == Mode::Title {
            self.remember_return();
        }
        self.arrival = INTRO_SECONDS;
        self.phase_two = false;
        self.phase_notice = 0.;
        self.pattern = Pattern::Fan;
        self.hit_flash = 0.;
        self.boss_flash = 0.;
        self.warning = false;
        self.particles.clear();
        self.duel = Duel::new();
        self.runner = Runner::new();
        self.platformer = crate::platformer::Platformer::new();
        self.race = crate::race::Race::new();
        self.mode = Mode::Battle;
        self.local_orbit = false;
        self.player = Vec3::new(-3., -0.5, 12.);
        self.health = 3;
        self.boss_hp = BOSS_MAX_HP;
        self.battle_time = 0.;
        self.shot_timer = 0.;
        self.enemy_timer = 2.6;
        self.wave = 0;
        self.invulnerable = 1.8;
        self.bullets.clear();
        self.velocity = Vec3::default();
        self.camera_eye = Vec3::new(-0.6, 3.6, 23.);
        self.camera_target = Vec3::new(-0.6, 2., 3.);
        if self.active_world == WorldId::Race {
            self.arrival = 0.;
            (self.camera_eye, self.camera_target) = crate::race_scene::camera(&self.race);
        } else if self.active_world == WorldId::Dojo {
            self.camera_eye = Vec3::new(0., 3.5, 15.);
            self.camera_target = Vec3::new(0., 1.8, 1.);
        } else if self.active_world == WorldId::Runner {
            self.camera_eye = Vec3::new(0., 6.3, 18.);
            self.camera_target = Vec3::new(0., 1.2, -18.);
        }
    }
    pub fn leave(&mut self) {
        if self.inside() && self.active_world == WorldId::Runner {
            self.progress.runner_score = self.progress.runner_score.max(self.runner.score());
        }
        self.mode = Mode::Flight;
        self.ship = self.world_at(self.active_world, self.return_offset);
        self.yaw = self.return_yaw;
        self.pitch = self.return_pitch;
        self.turn = Vec3::default();
        self.arrival = 0.7;
        self.camera_eye = self.ship - self.forward() * 9. + Vec3::new(0., 4., 0.);
        self.camera_target = self.ship + self.forward() * 12.;
        self.velocity = Vec3::default();
        self.bullets.clear();
        self.particles.clear();
        self.events.push(Cue::Portal);
    }
    pub fn tick(&mut self, i: Input, dt: f32) {
        if i.vibration {
            self.vibration_enabled = !self.vibration_enabled;
            if self.vibration_enabled {
                self.events.push(Cue::Block);
            }
        }
        if i.mute {
            self.audio_muted = !self.audio_muted;
        }
        if i.pause && self.mode != Mode::Title {
            self.paused = !self.paused;
        }
        if self.paused {
            return;
        }
        if self.mode == Mode::Title {
            if i.accept {
                self.mode = Mode::Flight
            }
            return;
        }
        let old_count = self.arrival.ceil() as i32;
        self.arrival = (self.arrival - dt).max(0.);
        if self.inside() && self.arrival.ceil() as i32 != old_count {
            self.events.push(Cue::Count);
        }
        self.hit_flash = (self.hit_flash - dt).max(0.);
        self.boss_flash = (self.boss_flash - dt).max(0.);
        self.phase_notice = (self.phase_notice - dt).max(0.);
        for p in &mut self.particles {
            p.p = p.p + p.velocity * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.);
        let old_planet = self.planet_at(self.active_world);
        self.time += dt;
        if i.destination && self.mode == Mode::Flight {
            self.active_world = self.active_world.other();
            self.local_orbit = false;
            self.events.push(Cue::Count);
        }
        if i.reset && self.mode == Mode::Flight {
            self.return_offset = Vec3::new(0., 2., 15.);
            self.return_yaw = -self.spin_at(self.active_world);
            self.return_pitch = 0.;
            self.leave();
        }
        match self.mode {
            Mode::Flight => {
                if i.accept && self.can_enter() {
                    self.begin_entry();
                } else {
                    if i.orbit && (self.ship - self.planet_at(self.active_world)).length() < 26. {
                        self.local_orbit = !self.local_orbit;
                        self.velocity = Vec3::default();
                    }
                    if self.local_orbit {
                        let rel = self.ship - old_planet;
                        let radius = (rel.x * rel.x + rel.z * rel.z).sqrt();
                        let new_radius = (radius - i.forward * 5. * dt).clamp(9., 24.);
                        let angle = rel.x.atan2(rel.z) + i.strafe * 0.8 * dt;
                        self.ship = self.planet_at(self.active_world)
                            + Vec3::new(
                                angle.sin() * new_radius,
                                (rel.y + i.lift * 5. * dt).clamp(-12., 12.),
                                angle.cos() * new_radius,
                            );
                        let dir = (self.planet_at(self.active_world) - self.ship).normalized();
                        self.yaw = dir.x.atan2(-dir.z);
                        self.pitch = dir.y.asin();
                    } else {
                        self.turn = self
                            .turn
                            .lerp(Vec3::new(i.yaw, i.pitch, 0.), 1. - (-10. * dt).exp());
                        self.yaw += self.turn.x * 1.5 * dt;
                        self.pitch = (self.pitch + self.turn.y * 1.2 * dt).clamp(-1.3, 1.3);
                        let forward = self.forward();
                        let right = forward.cross(Vec3::new(0., 1., 0.)).normalized();
                        let movement =
                            forward * i.forward + right * i.strafe + Vec3::new(0., i.lift, 0.);
                        let target =
                            movement * (if i.boost { 16. } else { 7. } / movement.length().max(1.));
                        self.velocity = self.velocity.lerp(
                            if i.brake { Vec3::default() } else { target },
                            1. - (-(if i.brake { 10. } else { 3. }) * dt).exp(),
                        );
                        self.ship = self.ship + self.velocity * dt;
                    }
                    self.resolve_collisions();
                }
            }
            Mode::Entering => {
                if i.back {
                    self.leave();
                } else {
                    self.portal_left = (self.portal_left - dt).max(0.);
                    let t = 1. - self.portal_left / 1.4;
                    let ease = t * t * (3. - 2. * t);
                    let end = self.gate()
                        - Vec3::new(0., 0., 2.).rotate_y(self.spin_at(self.active_world));
                    self.ship = self.portal_origin.lerp(end, ease);
                    if self.portal_left <= 0. {
                        self.enter();
                    }
                }
            }
            Mode::Battle => {
                if i.back {
                    self.leave()
                } else {
                    if self.active_world == WorldId::Race {
                        self.race.tick(i, dt, &mut self.events);
                        self.battle_time = self.race.time;
                        if self.race.phase == crate::race::Phase::Finished {
                            if self.race.rank == 1 {
                                self.win();
                            } else {
                                self.lose();
                            }
                        } else if self.race.lives <= 0 || self.race.time >= self.time_limit() {
                            self.lose();
                        }
                    } else if self.active_world == WorldId::Garden {
                        self.battle(i, dt);
                    } else if self.active_world == WorldId::Mario && self.arrival <= 0. {
                        self.battle_time += dt;
                        self.platformer.tick(i, dt, &mut self.events);
                        if self.platformer.finished {
                            self.win();
                        } else if self.platformer.lives <= 0
                            || self.battle_time >= self.time_limit()
                        {
                            self.lose();
                        }
                    } else if self.active_world == WorldId::Runner && self.arrival <= 0. {
                        self.battle_time += dt;
                        self.runner.tick(i, dt, &mut self.events);
                        if self.runner.health <= 0 {
                            self.lose();
                        } else if !self.runner.endless && self.runner.completed() {
                            self.win();
                        }
                    } else if self.active_world == WorldId::Dojo && self.arrival <= 0. {
                        self.battle_time += dt;
                        self.duel.tick(i, dt, &mut self.events);
                        if self.duel.enemy_hp <= 0 {
                            self.win();
                        } else if self.duel.player_hp <= 0 || self.battle_time >= self.time_limit()
                        {
                            self.lose();
                        }
                    }
                }
            }
            Mode::Victory | Mode::Defeat => {
                if self.mode == Mode::Victory && self.active_world == WorldId::Runner && i.fire {
                    self.runner.endless = true;
                    self.runner.invulnerable = 2.;
                    self.mode = Mode::Battle;
                } else if i.back {
                    self.leave()
                } else if i.accept {
                    if self.mode == Mode::Victory {
                        self.leave()
                    } else {
                        self.enter()
                    }
                }
            }
            Mode::Title => {}
        }
        let (eye, target) = if self.inside() && self.active_world == WorldId::Race {
            crate::race_scene::camera(&self.race)
        } else if self.inside() && self.active_world == WorldId::Mario {
            let x = self.platformer.camera;
            let y = 5. + (self.platformer.y - 4.).max(0.) * 0.55;
            (Vec3::new(x, y, 22.), Vec3::new(x, y, 0.))
        } else if self.inside() && self.active_world == WorldId::Runner {
            let x = self.runner.x * 0.18;
            (Vec3::new(x, 6.3, 18.), Vec3::new(x, 1.2, -18.))
        } else if self.inside() && self.active_world == WorldId::Dojo {
            let x = self.duel.player_x * 0.10;
            (Vec3::new(x, 3.5, 15.), Vec3::new(x, 1.8, 1.))
        } else if self.inside() {
            self.ship = self.player;
            let follow = Vec3::new(self.player.x * 0.22, self.player.y * 0.12, 0.);
            (
                follow + Vec3::new(0., 3.6, 23.),
                follow + Vec3::new(0., 2., 3.),
            )
        } else {
            let f = self.forward();
            (
                self.ship - f * 9. + Vec3::new(0., 4., 0.),
                self.ship + f * 12.,
            )
        };
        let blend = 1. - (-5. * dt).exp();
        self.camera_eye = self.camera_eye.lerp(eye, blend);
        self.camera_target = self.camera_target.lerp(target, blend);
    }
    fn resolve_collisions(&mut self) {
        // Substeps cap displacement well below the sphere radius, preventing tunnelling.
        for (center, radius) in [
            (self.planet(), 6.2),
            (self.planet_at(WorldId::Dojo), 6.6),
            (self.planet_at(WorldId::Runner), 6.4),
            (self.planet_at(WorldId::Mario), 6.6),
            (self.planet_at(WorldId::Race), 7.0),
            (Vec3::default(), 5.2),
        ] {
            let diff = self.ship - center;
            let distance = diff.length();
            if distance < radius {
                let normal = if distance < 0.001 {
                    Vec3::new(0., 0., 1.)
                } else {
                    diff * (1. / distance)
                };
                self.ship = center + normal * radius;
                let inward = self.velocity.dot(normal).min(0.);
                self.velocity = self.velocity - normal * inward;
                self.collisions += 1;
                if inward < -1. {
                    self.events.push(Cue::Hurt);
                }
            }
        }
    }
    fn battle(&mut self, i: Input, dt: f32) {
        if self.arrival > 0. {
            return;
        }
        self.battle_time += dt;
        self.invulnerable = (self.invulnerable - dt).max(0.);
        let movement = Vec3::new(i.strafe, i.forward + i.lift, 0.);
        self.player = self.player + movement * (6. * dt / movement.length().max(1.));
        self.player.x = self.player.x.clamp(-5.5, 5.5);
        self.player.y = self.player.y.clamp(-1.4, 5.2);
        self.shot_timer -= dt;
        self.enemy_timer -= dt;
        if i.fire && self.shot_timer <= 0. {
            self.shot_timer = 0.17;
            self.events.push(Cue::Shot);
            self.bullets.push(Bullet {
                p: self.player + Vec3::new(0., 0., -0.8),
                v: Vec3::new(0., 0., -18.),
                friendly: true,
                life: 1.,
            });
        }
        if !self.phase_two && self.boss_hp <= BOSS_MAX_HP / 2 {
            self.phase_two = true;
            self.phase_notice = 2.;
            self.enemy_timer = 1.4;
            self.warning = false;
            self.bullets.retain(|b| b.friendly);
            self.events.push(Cue::Phase);
        }
        let lead = if self.phase_two { 0.9 } else { 1.1 };
        if self.enemy_timer <= lead && !self.warning {
            self.warning = true;
            self.rain_gap = (((self.player.x + 5.) / 2.).round() as usize).min(5);
            self.events.push(Cue::Warning);
        }
        if self.enemy_timer <= 0. {
            self.emit_pattern();
            self.wave += 1;
            self.pattern = match self.wave % 3 {
                0 => Pattern::Fan,
                1 => Pattern::Ring,
                _ => Pattern::Rain,
            };
            self.enemy_timer = if self.phase_two { 2.15 } else { 2.8 };
            self.warning = false;
        }
        let boss = self.boss();
        let mut impacts = vec![];
        for b in &mut self.bullets {
            let previous = b.p;
            b.p = b.p + b.v * dt;
            b.life -= dt;
            if b.friendly {
                if segment_distance(previous, b.p, boss) < 1.45 {
                    b.life = 0.;
                    self.boss_hp -= 1;
                    self.boss_flash = 0.12;
                    impacts.push((b.p, [247, 214, 126]));
                    self.events.push(Cue::Hit);
                }
            } else if self.invulnerable <= 0. && segment_distance(previous, b.p, self.player) < 0.5
            {
                b.life = 0.;
                self.health -= 1;
                self.invulnerable = 1.5;
                self.hit_flash = 0.28;
                impacts.push((self.player, [255, 115, 93]));
                self.events.push(Cue::Hurt);
            }
        }
        for (p, color) in impacts {
            self.spark(p, color);
        }
        self.bullets
            .retain(|b| b.life > 0. && b.p.x.abs() < 12. && b.p.y.abs() < 10.);
        if self.boss_hp <= 0 {
            self.win();
        } else if self.health <= 0 || self.battle_time >= self.time_limit() {
            self.lose();
        }
    }
    fn emit_pattern(&mut self) {
        let origin = Vec3::new(self.boss().x, self.boss().y, 12.);
        let speed = if self.phase_two { 3.2 } else { 2.7 };
        let mut spawn = |p, v| {
            self.bullets.push(Bullet {
                p,
                v,
                friendly: false,
                life: 5.,
            })
        };
        match self.pattern {
            Pattern::Fan => {
                let aim = (self.player - origin).normalized();
                let a = aim.y.atan2(aim.x);
                let count = if self.phase_two { 2 } else { 1 };
                for k in -count..=count {
                    let angle = a + k as f32 * 0.30;
                    spawn(origin, Vec3::new(angle.cos(), angle.sin(), 0.) * speed);
                }
            }
            Pattern::Ring => {
                let count = if self.phase_two { 12 } else { 8 };
                for k in 0..count {
                    let a =
                        k as f32 * std::f32::consts::TAU / count as f32 + self.battle_time * 0.15;
                    spawn(origin, Vec3::new(a.cos(), a.sin(), 0.) * speed * 0.85);
                }
            }
            Pattern::Rain => {
                for lane in 0..6 {
                    if lane == self.rain_gap {
                        continue;
                    }
                    let x = -5. + lane as f32 * 2.;
                    spawn(Vec3::new(x, 6., 12.), Vec3::new(0., -speed, 0.));
                    if self.phase_two {
                        spawn(Vec3::new(x, 7., 12.), Vec3::new(0., -speed, 0.));
                    }
                }
            }
        }
    }
    fn spark(&mut self, p: Vec3, color: [u8; 3]) {
        for k in 0..8 {
            if self.particles.len() >= 96 {
                break;
            }
            let a = k as f32 * std::f32::consts::TAU / 8.;
            self.particles.push(Particle {
                p,
                velocity: Vec3::new(a.cos(), a.sin(), 0.) * 2.8,
                life: 0.35,
                color,
            });
        }
    }
    fn win(&mut self) {
        if self.active_world == WorldId::Runner {
            self.progress.runner_score = self.progress.runner_score.max(self.runner.score());
        }
        self.mode = Mode::Victory;
        self.progress
            .award(self.active_world.index(), self.battle_time);
        self.medal = self.progress.medals[0];
        self.bullets.clear();
        self.warning = false;
        self.events.push(Cue::Win);
    }
    fn lose(&mut self) {
        if self.active_world == WorldId::Runner {
            self.progress.runner_score = self.progress.runner_score.max(self.runner.score());
        }
        self.mode = Mode::Defeat;
        self.bullets.clear();
        self.warning = false;
        self.events.push(Cue::Lose);
    }
}
pub fn segment_distance(a: Vec3, b: Vec3, p: Vec3) -> f32 {
    let d = b - a;
    let t = ((p - a).dot(d) / d.dot(d).max(1e-8)).clamp(0., 1.);
    (p - (a + d * t)).length()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fifth_world_entry_race_medal_and_pause_preserve_other_worlds() {
        let mut g = Game::new();
        g.mode = Mode::Flight;
        g.active_world = WorldId::Race;
        g.ship = g.gate();
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Entering);
        for _ in 0..180 {
            g.tick(Input::default(), 1. / 120.);
        }
        assert_eq!(g.race.phase, crate::race::Phase::Choose);
        g.tick(
            Input {
                pause: true,
                ..Input::default()
            },
            1. / 120.,
        );
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1.,
        );
        assert_eq!(g.race.phase, crate::race::Phase::Choose);
        g.tick(
            Input {
                pause: true,
                ..Input::default()
            },
            1. / 120.,
        );
        for _ in 0..120 * 100 {
            let i = crate::race::route_input(&g.race);
            g.tick(i, 1. / 120.);
            g.events.clear();
            if g.mode != Mode::Battle {
                break;
            }
        }
        assert_eq!(g.mode, Mode::Victory);
        assert_eq!(g.progress.medals, [false, false, false, false, true]);
        assert!(g.race.time > crate::race::GOAL / 60. && g.race.time < 120.);
        assert_eq!(
            g.progress.best_ms[4],
            Some((g.race.time * 1000.).round() as u32)
        );
        let saved = crate::progress::Progress::decode(&g.progress.encode()).unwrap();
        assert_eq!(saved, g.progress);
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Flight);
    }
    #[test]
    fn orbit_keeps_radius_and_spin_continues_in_battle() {
        let mut g = Game::new();
        g.enter();
        for _ in 0..120 {
            g.tick(Input::default(), 1. / 120.);
        }
        assert!((g.planet().length() - 22.).abs() < 0.001);
        assert!(g.spin() > 0.03);
        assert!(g.planet().z > 0.3);
    }
    #[test]
    fn entry_range_and_return() {
        let mut g = Game::new();
        g.mode = Mode::Flight;
        g.ship = Vec3::new(100., 0., 0.);
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Flight);
        g.ship = g.gate();
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Entering);
        for _ in 0..180 {
            g.tick(Input::default(), 1. / 120.);
        }
        assert_eq!(g.mode, Mode::Battle);
        g.tick(
            Input {
                back: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Flight);
        assert!((g.ship - g.planet()).length() > 6.2);
    }
    #[test]
    fn collision_prevents_crossing() {
        let mut g = Game::new();
        g.mode = Mode::Flight;
        g.ship = g.planet();
        g.resolve_collisions();
        assert!((g.ship - g.planet()).length() >= 6.19);
    }
    #[test]
    fn victory_retry_and_failure() {
        let mut g = Game::new();
        g.enter();
        g.arrival = 0.;
        g.boss_hp = 1;
        let b = g.boss();
        g.bullets.push(Bullet {
            p: b + Vec3::new(0., 0., 0.2),
            v: Vec3::new(0., 0., -18.),
            friendly: true,
            life: 1.,
        });
        g.tick(Input::default(), 1. / 120.);
        assert_eq!(g.mode, Mode::Victory);
        assert!(g.medal);
        g.leave();
        g.enter();
        g.arrival = 0.;
        g.health = 0;
        g.tick(Input::default(), 1. / 120.);
        assert_eq!(g.mode, Mode::Defeat);
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.health, 3);
        assert_eq!(g.boss_hp, BOSS_MAX_HP);
        assert_eq!(g.mode, Mode::Battle);
    }
    #[test]
    fn pause_freezes_and_does_not_consume_accept() {
        let mut g = Game::new();
        g.mode = Mode::Flight;
        g.tick(
            Input {
                pause: true,
                ..Input::default()
            },
            0.1,
        );
        let t = g.time;
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            0.1,
        );
        assert_eq!(g.time, t);
        assert_eq!(g.mode, Mode::Flight);
    }
    #[test]
    fn swept_projectile_catches_pass_through() {
        assert!(
            segment_distance(
                Vec3::new(-2., 0., 0.),
                Vec3::new(2., 0., 0.),
                Vec3::default()
            ) < 0.001
        );
    }
    #[test]
    fn complete_challenge_with_movement_and_shooting() {
        // Exercise real shots, enemy patterns, damage and victory without editing HP.
        let mut g = Game::new();
        g.enter();
        for _ in 0..7200 {
            if g.mode != Mode::Battle {
                break;
            }
            let angle = g.battle_time * 2.4;
            let target = g.boss() + Vec3::new(angle.cos(), angle.sin(), 5.);
            let movement = (target - g.player) * 3.;
            g.tick(
                Input {
                    strafe: movement.x.clamp(-1., 1.),
                    forward: movement.y.clamp(-1., 1.),
                    fire: true,
                    ..Input::default()
                },
                1. / 120.,
            );
        }
        assert_eq!(
            g.mode,
            Mode::Victory,
            "boss HP {}, player HP {}, time {}",
            g.boss_hp,
            g.health,
            g.battle_time
        );
        assert!(g.medal);
    }
    #[test]
    fn orbit_uses_elapsed_time() {
        let mut a = Game::new();
        let mut b = Game::new();
        a.mode = Mode::Flight;
        b.mode = Mode::Flight;
        for _ in 0..300 {
            a.tick(Input::default(), 1. / 30.)
        }
        for _ in 0..1200 {
            b.tick(Input::default(), 1. / 120.)
        }
        assert!((a.planet() - b.planet()).length() < 0.002);
    }
    #[test]
    fn idle_chase_camera_does_not_orbit() {
        let mut g = Game::new();
        g.mode = Mode::Flight;
        let eye = g.camera_eye;
        let heading = g.camera_target - g.camera_eye;
        for _ in 0..600 {
            g.tick(Input::default(), 1. / 120.);
        }
        assert!((g.camera_eye - eye).length() < 0.001);
        assert!((g.camera_target - g.camera_eye - heading).length() < 0.001);
    }
    #[test]
    fn interior_camera_translates_without_turning() {
        let mut g = Game::new();
        g.enter();
        let heading = g.camera_target - g.camera_eye;
        for _ in 0..480 {
            g.tick(
                Input {
                    strafe: 0.3,
                    ..Input::default()
                },
                1. / 120.,
            );
        }
        assert!((g.camera_target - g.camera_eye - heading).length() < 0.002);
        assert!(g.camera_eye.x > -0.6);
    }
    #[test]
    fn garden_warns_before_all_patterns_and_latches_phase_two() {
        let mut g = Game::new();
        g.enter();
        g.arrival = 0.;
        for expected in [Pattern::Fan, Pattern::Ring, Pattern::Rain] {
            assert_eq!(g.pattern, expected);
            g.enemy_timer = 0.8;
            g.tick(Input::default(), 1. / 120.);
            assert!(g.warning);
            assert!(g.events.contains(&Cue::Warning));
            g.events.clear();
            let count = g.bullets.len();
            g.enemy_timer = 0.001;
            g.tick(Input::default(), 1. / 120.);
            assert!(g.bullets.len() > count);
            assert!(!g.warning);
        }
        g.boss_hp = BOSS_MAX_HP / 2;
        g.tick(Input::default(), 1. / 120.);
        assert!(g.phase_two);
        assert!(g.events.contains(&Cue::Phase));
        assert!(g.bullets.iter().all(|b| b.friendly));
        g.events.clear();
        g.tick(Input::default(), 1. / 120.);
        assert!(!g.events.contains(&Cue::Phase));
    }
    #[test]
    fn second_world_portal_duel_award_and_return() {
        use crate::duel::{Attack, Stage};
        let mut g = Game::new();
        g.mode = Mode::Flight;
        let initial_ship = g.ship;
        g.tick(
            Input {
                destination: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.active_world, WorldId::Dojo);
        assert!((g.ship - initial_ship).length() < 0.001);
        g.ship = g.gate();
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Entering);
        for _ in 0..120 * 60 {
            if g.mode == Mode::Victory || g.mode == Mode::Defeat {
                break;
            }
            g.tick(
                Input {
                    brake: g.duel.attack != Attack::Sweep,
                    dodge: g.duel.stage == Stage::Telegraph
                        && g.duel.attack == Attack::Sweep
                        && g.duel.timer < 0.2,
                    fire: g.duel.stage == Stage::Recovery,
                    ..Input::default()
                },
                1. / 120.,
            );
        }
        assert_eq!(g.mode, Mode::Victory);
        assert_eq!(g.progress.medals, [false, true, false, false, false]);
        assert!(g.progress.best_ms[1].is_some());
        g.tick(
            Input {
                accept: true,
                ..Input::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Flight);
        assert!((g.ship - g.planet_at(WorldId::Dojo)).length() > 6.6);
    }
    #[test]
    fn runner_route_awards_third_medal_and_continues_endlessly() {
        let mut g = Game::new();
        g.active_world = WorldId::Runner;
        g.mode = Mode::Flight;
        g.ship = g.gate();
        g.tick(
            Input {
                accept: true,
                ..Default::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Entering);
        for step in 0..120 * 100 {
            let i = crate::runner::route_input(&g.runner, step % 2 == 0);
            g.tick(i, 1. / 120.);
            if matches!(g.mode, Mode::Victory | Mode::Defeat) {
                break;
            }
        }
        assert_eq!(
            g.mode,
            Mode::Victory,
            "distance {} health {} coins {}",
            g.runner.distance,
            g.runner.health,
            g.runner.coins
        );
        assert_eq!(g.progress.medals, [false, false, true, false, false]);
        assert!(g.progress.runner_score > 900);
        let record = g.progress.best_ms[2];
        g.tick(
            Input {
                fire: true,
                ..Default::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Battle);
        assert!(g.runner.endless);
        for step in 0..120 * 15 {
            let i = crate::runner::route_input(&g.runner, step % 2 == 0);
            g.tick(i, 1. / 120.);
        }
        assert_eq!(g.mode, Mode::Battle);
        assert!(g.runner.distance > 1100.);
        let score = g.runner.score();
        g.tick(
            Input {
                back: true,
                ..Default::default()
            },
            1. / 120.,
        );
        assert_eq!(g.mode, Mode::Flight);
        assert_eq!(g.progress.runner_score, score);
        assert_eq!(g.progress.best_ms[2], record);
        assert!((g.ship - g.planet_at(WorldId::Runner)).length() > 6.4);
    }
    #[test]
    fn runner_failure_retry_and_pause_preserve_rules() {
        let mut g = Game::new();
        g.active_world = WorldId::Runner;
        g.enter();
        for _ in 0..120 * 100 {
            g.tick(Input::default(), 1. / 120.);
            if g.mode == Mode::Defeat {
                break;
            }
        }
        assert_eq!(g.mode, Mode::Defeat);
        assert!(!g.progress.medals[2]);
        assert!(g.progress.runner_score > 0);
        g.tick(
            Input {
                accept: true,
                ..Default::default()
            },
            1. / 120.,
        );
        assert_eq!(g.runner.health, 3);
        assert_eq!(g.runner.distance, 0.);
        assert_eq!(g.arrival, INTRO_SECONDS);
        g.tick(
            Input {
                pause: true,
                ..Default::default()
            },
            1. / 120.,
        );
        let arrival = g.arrival;
        for _ in 0..120 {
            g.tick(
                Input {
                    jump: true,
                    strafe: 1.,
                    ..Default::default()
                },
                1. / 120.,
            );
        }
        assert_eq!(g.runner.distance, 0.);
        assert_eq!(g.arrival, arrival);
    }
}
