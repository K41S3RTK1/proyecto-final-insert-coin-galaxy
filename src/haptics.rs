//! Native controller haptics; GLFW's raylib vibration entry point is a no-op.
use crate::feedback::Cue;
#[derive(Clone, Copy, Debug, PartialEq)]
struct Pulse {
    power: f32,
    sharpness: f32,
    seconds: f32,
    priority: u8,
}
fn pulse(events: &[Cue]) -> Option<Pulse> {
    events
        .iter()
        .filter_map(|cue| {
            let (power, sharpness, seconds, priority) = match cue {
                Cue::Hurt => (0.85, 0.3, 0.25, 9),
                Cue::Lose => (0.65, 0.1, 0.45, 10),
                Cue::Win => (0.65, 0.6, 0.55, 11),
                Cue::Phase => (0.55, 0.2, 0.30, 7),
                Cue::Portal => (0.45, 0.5, 0.22, 6),
                Cue::Block => (0.50, 0.8, 0.10, 5),
                Cue::Hit => (0.32, 0.8, 0.07, 4),
                Cue::Dodge | Cue::Jump => (0.18, 0.5, 0.06, 3),
                Cue::Punch | Cue::Shot => (0.15, 0.7, 0.035, 2),
                Cue::Coin => (0.12, 0.9, 0.025, 1),
                _ => return None,
            };
            Some(Pulse {
                power,
                sharpness,
                seconds,
                priority,
            })
        })
        .max_by_key(|p| p.priority)
}
#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn arcade_haptics_status() -> i32;
    fn arcade_haptics_play(power: f32, sharpness: f32, duration: f32) -> i32;
    fn arcade_haptics_stop();
    fn arcade_haptics_shutdown();
}
#[derive(Default)]
pub struct Haptics {
    pub available: bool,
    pub delivered: u32,
    check_in: f32,
    remaining: f32,
    priority: u8,
    active: bool,
}
impl Haptics {
    pub fn update(
        &mut self,
        connected: bool,
        enabled: bool,
        paused: bool,
        dt: f32,
        events: &[Cue],
    ) {
        self.check_in -= dt;
        self.remaining = (self.remaining - dt).max(0.);
        if !connected {
            self.stop();
            self.available = false;
            self.check_in = 0.;
            return;
        }
        if self.check_in <= 0. {
            self.check_in = 1.;
            #[cfg(target_os = "macos")]
            {
                self.available = unsafe { arcade_haptics_status() != 0 };
            }
        }
        if !enabled || paused || !self.available {
            self.stop();
            return;
        }
        if let Some(p) = pulse(events)
            && (self.remaining <= 0. || p.priority > self.priority)
        {
            #[cfg(target_os = "macos")]
            let played = unsafe { arcade_haptics_play(p.power, p.sharpness, p.seconds) != 0 };
            #[cfg(not(target_os = "macos"))]
            let played = false;
            if played {
                self.delivered = self.delivered.saturating_add(1);
                self.remaining = p.seconds;
                self.priority = p.priority;
                self.active = true;
            } else {
                self.available = false;
            }
        }
    }
    pub fn stop(&mut self) {
        if self.active {
            #[cfg(target_os = "macos")]
            unsafe {
                arcade_haptics_stop();
            }
        }
        self.active = false;
        self.remaining = 0.;
        self.priority = 0;
    }
}
impl Drop for Haptics {
    fn drop(&mut self) {
        self.stop();
        #[cfg(target_os = "macos")]
        unsafe {
            arcade_haptics_shutdown();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn damage_and_victory_override_small_pulses() {
        let hurt = pulse(&[Cue::Shot, Cue::Coin, Cue::Hurt]).unwrap();
        assert_eq!(hurt, pulse(&[Cue::Hurt]).unwrap());
        assert_eq!(pulse(&[Cue::Hurt, Cue::Win]), pulse(&[Cue::Win]));
        assert_eq!(pulse(&[Cue::Warning, Cue::Count]), None);
    }
    #[test]
    fn pause_and_disconnect_clear_active_pulse_without_hardware() {
        let mut h = Haptics {
            remaining: 0.5,
            priority: 9,
            ..Default::default()
        };
        h.update(false, true, false, 0.01, &[Cue::Hurt]);
        assert_eq!(h.remaining, 0.);
        h.remaining = 0.2;
        h.check_in = 1.;
        h.available = true;
        h.update(true, true, true, 0.01, &[Cue::Win]);
        assert_eq!(h.remaining, 0.);
        assert!(!h.active);
    }
}
