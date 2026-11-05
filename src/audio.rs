use crate::{
    feedback::Cue,
    game::{Game, Mode},
};
use raylib::prelude::*;
pub struct Audio<'a> {
    sounds: Vec<(Cue, Sound<'a>)>,
    engine: Sound<'a>,
}
impl<'a> Audio<'a> {
    pub fn new(device: &'a RaylibAudio, root: &std::path::Path) -> Result<Self, String> {
        let mut sounds = vec![];
        for cue in [
            Cue::Coin,
            Cue::Jump,
            Cue::Shot,
            Cue::Hit,
            Cue::Hurt,
            Cue::Win,
            Cue::Lose,
            Cue::Portal,
            Cue::Count,
            Cue::Warning,
            Cue::Phase,
            Cue::Block,
            Cue::Dodge,
            Cue::Punch,
        ] {
            let sound = device
                .new_sound(&root.join(format!("{}.wav", cue.file())).to_string_lossy())
                .map_err(|e| e.to_string())?;
            sound.set_volume(if cue == Cue::Shot { 0.20 } else { 0.40 });
            sounds.push((cue, sound));
        }
        let engine = device
            .new_sound(&root.join("engine.wav").to_string_lossy())
            .map_err(|e| e.to_string())?;
        Ok(Self { sounds, engine })
    }
    pub fn update(&self, g: &Game, events: &[Cue]) {
        if g.audio_muted || g.paused {
            for (_, sound) in &self.sounds {
                sound.stop();
            }
            self.engine.stop();
            return;
        }
        for (cue, sound) in &self.sounds {
            if events.contains(cue) {
                sound.play();
            }
        }
        if g.mode == Mode::Flight || g.mode == Mode::Entering {
            let speed = (g.velocity.length() / 16.).clamp(0., 1.);
            self.engine.set_volume(0.08 + speed * 0.12);
            self.engine.set_pitch(0.8 + speed * 0.65);
            if !self.engine.is_playing() {
                self.engine.play();
            }
        } else {
            self.engine.stop();
        }
    }
}
