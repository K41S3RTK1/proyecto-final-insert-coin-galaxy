use raylib::prelude::*;
#[derive(Default, Clone, Copy, Debug)]
pub struct Input {
    pub race_boost: bool,
    pub race_ability: bool,
    pub forward: f32,
    pub strafe: f32,
    pub lift: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fire: bool,
    pub brake: bool,
    pub boost: bool,
    pub accept: bool,
    pub back: bool,
    pub pause: bool,
    pub orbit: bool,
    pub reset: bool,
    pub dodge: bool,
    pub destination: bool,
    pub mute: bool,
    pub quality: bool,
    pub jump: bool,
    pub slide: bool,
    pub vibration: bool,
}
impl Input {
    pub fn merge_edges(&mut self, other: Self) {
        self.race_ability |= other.race_ability;
        self.jump |= other.jump;
        self.slide |= other.slide;
        self.vibration |= other.vibration;
        self.accept |= other.accept;
        self.back |= other.back;
        self.pause |= other.pause;
        self.orbit |= other.orbit;
        self.reset |= other.reset;
        self.dodge |= other.dodge;
        self.destination |= other.destination;
        self.mute |= other.mute;
        self.quality |= other.quality;
    }

    pub fn clear_edges(&mut self) {
        self.race_ability = false;
        self.jump = false;
        self.slide = false;
        self.vibration = false;
        self.accept = false;
        self.back = false;
        self.pause = false;
        self.orbit = false;
        self.reset = false;
        self.dodge = false;
        self.destination = false;
        self.mute = false;
        self.quality = false;
    }
}
pub fn deadzone(value: f32) -> f32 {
    if value.abs() < 0.18 {
        0.
    } else {
        value.signum() * ((value.abs() - 0.18) / 0.82).clamp(0., 1.)
    }
}
pub struct Controls {
    pub gamepad: Option<i32>,
    pub name: String,
    pub using_pad: bool,
}
impl Controls {
    pub fn new() -> Self {
        Self {
            gamepad: None,
            name: String::new(),
            using_pad: false,
        }
    }
    pub fn read(&mut self, rl: &RaylibHandle) -> Input {
        use GamepadAxis::*;
        use GamepadButton::*;
        use KeyboardKey::*;
        self.gamepad = (0..4).find(|i| rl.is_gamepad_available(*i));
        self.name = self
            .gamepad
            .and_then(|id| rl.get_gamepad_name(id))
            .unwrap_or_default();
        let key = |k| if rl.is_key_down(k) { 1. } else { 0. };
        let mut i = Input {
            race_boost: rl.is_key_down(KEY_LEFT_SHIFT),
            race_ability: rl.is_key_pressed(KEY_L),
            forward: key(KEY_W) - key(KEY_S),
            strafe: key(KEY_D) - key(KEY_A),
            lift: key(KEY_R) - key(KEY_F),
            yaw: key(KEY_RIGHT) - key(KEY_LEFT),
            pitch: key(KEY_UP) - key(KEY_DOWN),
            fire: rl.is_key_down(KEY_SPACE) || rl.is_key_down(KEY_J),
            brake: rl.is_key_down(KEY_LEFT_CONTROL) || rl.is_key_down(KEY_K),
            boost: rl.is_key_down(KEY_LEFT_SHIFT),
            accept: rl.is_key_pressed(KEY_E) || rl.is_key_pressed(KEY_ENTER),
            back: rl.is_key_pressed(KEY_Q),
            pause: rl.is_key_pressed(KEY_P) || rl.is_key_pressed(KEY_ESCAPE),
            orbit: rl.is_key_pressed(KEY_TAB),
            reset: rl.is_key_pressed(KEY_HOME),
            dodge: rl.is_key_pressed(KEY_L) || rl.is_key_pressed(KEY_LEFT_SHIFT),
            destination: rl.is_key_pressed(KEY_N),
            mute: rl.is_key_pressed(KEY_M),
            quality: rl.is_key_pressed(KEY_F4),
            jump: rl.is_key_pressed(KEY_SPACE)
                || rl.is_key_pressed(KEY_W)
                || rl.is_key_pressed(KEY_UP),
            slide: rl.is_key_pressed(KEY_S) || rl.is_key_pressed(KEY_DOWN),
            vibration: rl.is_key_pressed(KEY_F5),
        };
        if i.forward != 0.
            || i.strafe != 0.
            || i.lift != 0.
            || i.yaw != 0.
            || i.pitch != 0.
            || i.fire
            || i.accept
            || i.back
            || i.pause
            || i.orbit
            || i.brake
            || i.boost
            || i.dodge
            || i.destination
            || i.mute
            || i.quality
            || i.jump
            || i.slide
            || i.vibration
        {
            self.using_pad = false;
        }
        if let Some(id) = self.gamepad {
            let axis = |a| deadzone(rl.get_gamepad_axis_movement(id, a));
            let down = |b| rl.is_gamepad_button_down(id, b);
            let pressed = |b| rl.is_gamepad_button_pressed(id, b);
            i.race_boost |= down(GAMEPAD_BUTTON_RIGHT_TRIGGER_2) || down(GAMEPAD_BUTTON_LEFT_THUMB);
            let sx = axis(GAMEPAD_AXIS_LEFT_X);
            let sy = axis(GAMEPAD_AXIS_LEFT_Y);
            let rx = axis(GAMEPAD_AXIS_RIGHT_X);
            let ry = axis(GAMEPAD_AXIS_RIGHT_Y);
            let fire = down(GAMEPAD_BUTTON_RIGHT_TRIGGER_2) || down(GAMEPAD_BUTTON_RIGHT_FACE_LEFT);
            let accept = pressed(GAMEPAD_BUTTON_RIGHT_FACE_DOWN);
            let back = pressed(GAMEPAD_BUTTON_RIGHT_FACE_RIGHT);
            let pause = pressed(GAMEPAD_BUTTON_MIDDLE_RIGHT);
            let orbit = pressed(GAMEPAD_BUTTON_RIGHT_FACE_UP);
            let dodge = pressed(GAMEPAD_BUTTON_RIGHT_TRIGGER_1);
            let destination = pressed(GAMEPAD_BUTTON_RIGHT_THUMB);
            let mute = pressed(GAMEPAD_BUTTON_MIDDLE_LEFT);
            let quality = pressed(GAMEPAD_BUTTON_LEFT_FACE_DOWN);
            let slide = pressed(GAMEPAD_BUTTON_LEFT_TRIGGER_2);
            let vibration = pressed(GAMEPAD_BUTTON_LEFT_FACE_UP);
            let lift = if down(GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
                1.
            } else {
                0.
            } - if down(GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
                1.
            } else {
                0.
            };
            if sx != 0.
                || sy != 0.
                || rx != 0.
                || ry != 0.
                || fire
                || accept
                || back
                || pause
                || orbit
                || dodge
                || destination
                || mute
                || quality
                || slide
                || vibration
                || lift != 0.
                || down(GAMEPAD_BUTTON_LEFT_TRIGGER_2)
                || down(GAMEPAD_BUTTON_LEFT_THUMB)
            {
                self.using_pad = true;
            }
            i.strafe = (i.strafe + sx).clamp(-1., 1.);
            i.forward = (i.forward - sy).clamp(-1., 1.);
            i.yaw = (i.yaw + rx).clamp(-1., 1.);
            i.pitch = (i.pitch - ry).clamp(-1., 1.);
            i.lift = (i.lift + lift).clamp(-1., 1.);
            i.fire |= fire;
            i.accept |= accept;
            i.back |= back;
            i.pause |= pause;
            i.orbit |= orbit;
            i.dodge |= dodge;
            i.race_ability |= dodge;
            i.destination |= destination;
            i.mute |= mute;
            i.quality |= quality;
            i.jump |= accept;
            i.slide |= slide;
            i.vibration |= vibration;
            i.brake |= down(GAMEPAD_BUTTON_LEFT_TRIGGER_2);
            i.boost |= down(GAMEPAD_BUTTON_LEFT_THUMB);
        } else {
            self.using_pad = false;
        }
        i
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stick_drift_and_full_range() {
        for v in [-0.17, 0., 0.17] {
            assert_eq!(deadzone(v), 0.)
        }
        assert!((deadzone(1.) - 1.).abs() < 1e-6);
        assert!((deadzone(-1.) + 1.).abs() < 1e-6);
        assert!((deadzone(0.59) - 0.5).abs() < 1e-5);
    }
}
