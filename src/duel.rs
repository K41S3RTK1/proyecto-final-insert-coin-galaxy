//! Small defensive fighting game: read a tell, defend, then punish recovery.
use crate::{feedback::Cue, input::Input};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attack {
    Jab,
    Sweep,
    Wave,
}
impl Attack {
    pub fn hint(self) -> &'static str {
        match self {
            Self::Jab => "PUNO / BLOQUEA",
            Self::Sweep => "BARRIDO / ESQUIVA",
            Self::Wave => "ENERGIA / BLOQUEA O ESQUIVA",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Rest,
    Telegraph,
    Strike,
    Recovery,
}
pub struct Duel {
    pub stage: Stage,
    pub attack: Attack,
    pub timer: f32,
    pub player_x: f32,
    pub enemy_x: f32,
    pub player_hp: i32,
    pub enemy_hp: i32,
    pub blocking: bool,
    pub dodge: f32,
    pub dodge_cooldown: f32,
    pub attack_pose: f32,
    pub hit_flash: f32,
    pub counter_open: bool,
    pub message: &'static str,
    sequence: u32,
    punch_cooldown: f32,
}
impl Duel {
    pub fn new() -> Self {
        Self {
            stage: Stage::Rest,
            attack: Attack::Jab,
            timer: 0.8,
            player_x: -2.,
            enemy_x: 2.,
            player_hp: 5,
            enemy_hp: 6,
            blocking: false,
            dodge: 0.,
            dodge_cooldown: 0.,
            attack_pose: 0.,
            hit_flash: 0.,
            counter_open: false,
            message: "LEE EL ATAQUE Y DEFIENDE",
            sequence: 0,
            punch_cooldown: 0.,
        }
    }
    pub fn phase_two(&self) -> bool {
        self.enemy_hp <= 3
    }
    pub fn tick(&mut self, i: Input, dt: f32, events: &mut Vec<Cue>) {
        self.dodge = (self.dodge - dt).max(0.);
        self.dodge_cooldown = (self.dodge_cooldown - dt).max(0.);
        self.attack_pose = (self.attack_pose - dt).max(0.);
        self.hit_flash = (self.hit_flash - dt).max(0.);
        self.punch_cooldown = (self.punch_cooldown - dt).max(0.);
        self.blocking = i.brake && self.dodge <= 0.;
        self.player_x =
            (self.player_x + i.strafe * dt * if self.blocking { 1.3 } else { 3.6 }).clamp(-5., 0.3);
        if i.dodge && self.dodge_cooldown <= 0. {
            self.dodge = 0.38;
            self.dodge_cooldown = 0.8;
            self.blocking = false;
            events.push(Cue::Dodge);
        }
        self.timer -= dt;
        if self.timer <= 0. {
            match self.stage {
                Stage::Rest => {
                    self.stage = Stage::Telegraph;
                    self.attack = match self.sequence % 3 {
                        0 => Attack::Jab,
                        1 => Attack::Sweep,
                        _ => Attack::Wave,
                    };
                    self.sequence += 1;
                    self.timer = if self.phase_two() { 0.72 } else { 1.0 };
                    self.message = self.attack.hint();
                    events.push(Cue::Warning);
                }
                Stage::Telegraph => {
                    self.stage = Stage::Strike;
                    self.timer = 0.18;
                    let defended = match self.attack {
                        Attack::Jab => self.blocking,
                        Attack::Sweep => self.dodge > 0.,
                        Attack::Wave => self.blocking || self.dodge > 0.,
                    };
                    let in_range =
                        self.enemy_x - self.player_x < 5.2 || self.attack == Attack::Wave;
                    self.counter_open = defended;
                    if defended {
                        self.message = "BIEN! AHORA CONTRAATACA";
                        events.push(if self.dodge > 0. {
                            Cue::Dodge
                        } else {
                            Cue::Block
                        });
                    } else if in_range {
                        self.player_hp -= 1;
                        self.hit_flash = 0.35;
                        self.message = "GOLPE RECIBIDO";
                        events.push(Cue::Hurt);
                    } else {
                        self.message = "ACERCATE PARA CONTRAATACAR";
                    }
                }
                Stage::Strike => {
                    self.stage = Stage::Recovery;
                    self.timer = 1.05;
                }
                Stage::Recovery => {
                    self.stage = Stage::Rest;
                    self.counter_open = false;
                    self.timer = 0.6;
                    self.message = "PREPARA LA SIGUIENTE DEFENSA";
                }
            }
        }
        if i.fire && self.punch_cooldown <= 0. && self.dodge <= 0. {
            self.punch_cooldown = 0.38;
            self.attack_pose = 0.24;
            events.push(Cue::Punch);
            if self.stage == Stage::Recovery
                && self.counter_open
                && self.enemy_x - self.player_x < 5.0
            {
                self.enemy_hp -= 1;
                self.counter_open = false;
                self.hit_flash = 0.28;
                self.message = "CONTRAATAQUE!";
                events.push(Cue::Hit);
            } else if self.stage == Stage::Telegraph {
                self.message = "PRIMERO DEFIENDE";
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn successful_duel_requires_defense_and_counter() {
        let mut d = Duel::new();
        let mut cues = vec![];
        for _ in 0..120 * 60 {
            if d.enemy_hp == 0 || d.player_hp == 0 {
                break;
            }
            let dodge = d.stage == Stage::Telegraph && d.attack == Attack::Sweep && d.timer < 0.20;
            d.tick(
                Input {
                    brake: d.attack != Attack::Sweep,
                    dodge,
                    fire: d.stage == Stage::Recovery,
                    ..Input::default()
                },
                1. / 120.,
                &mut cues,
            );
        }
        assert_eq!(d.enemy_hp, 0);
        assert_eq!(d.player_hp, 5);
        assert!(cues.contains(&Cue::Block));
        assert!(cues.contains(&Cue::Dodge));
    }
    #[test]
    fn holding_attack_cannot_win() {
        let mut d = Duel::new();
        let mut cues = vec![];
        for _ in 0..120 * 30 {
            d.tick(
                Input {
                    fire: true,
                    ..Input::default()
                },
                1. / 120.,
                &mut cues,
            );
            if d.player_hp <= 0 {
                break;
            }
        }
        assert!(d.player_hp <= 0);
        assert_eq!(d.enemy_hp, 6);
    }
    #[test]
    fn blocking_cannot_stop_sweep() {
        let mut d = Duel::new();
        d.attack = Attack::Sweep;
        d.stage = Stage::Telegraph;
        d.timer = 0.001;
        d.tick(
            Input {
                brake: true,
                ..Input::default()
            },
            0.01,
            &mut vec![],
        );
        assert_eq!(d.player_hp, 4);
    }
}
