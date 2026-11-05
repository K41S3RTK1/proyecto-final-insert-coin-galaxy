use crate::{
    game::Game,
    math::Vec3,
    renderer::{Renderer, Style},
    runner::{Kind, PLAYER_Z},
    scene::Scene,
};
use std::f32::consts::{PI, TAU};
const YELLOW: [u8; 3] = [251, 207, 48];
const BLUE: [u8; 3] = [48, 108, 179];
const DARK: [u8; 3] = [39, 44, 58];

// A modeled Minion skin: yellow capsule, denim overalls, two silver goggles,
// black strap/gloves/boots and sparse hair. No downloaded model or sprite.
pub fn minion(
    s: &Scene,
    r: &mut Renderer,
    root: Vec3,
    scale: f32,
    yaw: f32,
    time: f32,
    slide: bool,
) {
    let pose = |v: Vec3| {
        let v = if slide {
            v.rotate_x(-PI * 0.40) + Vec3::new(0., 0.12, 0.)
        } else {
            v
        };
        root + (v * scale).rotate_y(yaw)
    };
    let mut ball = |v: Vec3, size: Vec3, color, style| {
        let size = if slide {
            Vec3::new(size.x, size.z, size.y)
        } else {
            size
        };
        s.ball(r, pose(v), size * scale, yaw, color, style);
    };
    let swing = (time * 13.).sin() * if slide { 0. } else { 0.28 };
    ball(
        Vec3::new(0., 1.35, 0.),
        Vec3::new(0.62, 0.90, 0.48),
        YELLOW,
        Style::Soft,
    );
    ball(
        Vec3::new(0., 0.77, 0.),
        Vec3::new(0.64, 0.48, 0.49),
        BLUE,
        Style::Soft,
    );
    // Front and back bib, straps and two gold buttons.
    ball(
        Vec3::new(0., 1.05, 0.43),
        Vec3::new(0.40, 0.32, 0.07),
        BLUE,
        Style::Ink,
    );
    ball(
        Vec3::new(0., 1.05, -0.43),
        Vec3::new(0.40, 0.31, 0.07),
        BLUE,
        Style::Soft,
    );
    for side in [-1., 1.] {
        ball(
            Vec3::new(side * 0.37, 1.27, 0.39),
            Vec3::new(0.10, 0.23, 0.08),
            BLUE,
            Style::Soft,
        );
        ball(
            Vec3::new(side * 0.34, 1.15, 0.51),
            Vec3::new(0.05, 0.05, 0.025),
            [240, 192, 84],
            Style::Glow,
        );
        ball(
            Vec3::new(side * 0.28, 0.32, side * swing),
            Vec3::new(0.23, 0.30, 0.21),
            BLUE,
            Style::Soft,
        );
        ball(
            Vec3::new(side * 0.28, 0.12, 0.10 + side * swing),
            Vec3::new(0.28, 0.15, 0.36),
            DARK,
            Style::Soft,
        );
        ball(
            Vec3::new(side * 0.68, 1.04, -side * swing),
            Vec3::new(0.15, 0.38, 0.15),
            YELLOW,
            Style::Soft,
        );
        ball(
            Vec3::new(side * 0.73, 0.78, -side * swing),
            Vec3::new(0.22, 0.23, 0.21),
            DARK,
            Style::Soft,
        );
    }
    // Strap wraps the entire head; face looks back playfully during the run.
    ball(
        Vec3::new(0., 1.74, -0.03),
        Vec3::new(0.635, 0.115, 0.48),
        DARK,
        Style::Soft,
    );
    for side in [-1., 1.] {
        ball(
            Vec3::new(side * 0.285, 1.77, 0.41),
            Vec3::new(0.30, 0.31, 0.20),
            [177, 190, 197],
            Style::Metal,
        );
        ball(
            Vec3::new(side * 0.285, 1.77, 0.58),
            Vec3::new(0.225, 0.235, 0.04),
            [249, 246, 218],
            Style::Soft,
        );
        ball(
            Vec3::new(side * 0.285 + 0.035, 1.77, 0.625),
            Vec3::new(0.095, 0.105, 0.025),
            [116, 75, 37],
            Style::Soft,
        );
        ball(
            Vec3::new(side * 0.285 + 0.035, 1.77, 0.65),
            Vec3::new(0.045, 0.065, 0.018),
            DARK,
            Style::Glow,
        );
        ball(
            Vec3::new(side * 0.285 + 0.01, 1.81, 0.67),
            Vec3::new(0.025, 0.025, 0.01),
            [255, 255, 240],
            Style::Glow,
        );
    }
    ball(
        Vec3::new(0., 1.38, 0.46),
        Vec3::new(0.19, 0.085, 0.035),
        [94, 65, 37],
        Style::Soft,
    );
    ball(
        Vec3::new(0., 1.41, 0.49),
        Vec3::new(0.12, 0.026, 0.013),
        [249, 243, 222],
        Style::Glow,
    );
    for k in -2..=2 {
        ball(
            Vec3::new(k as f32 * 0.12, 2.20, 0.),
            Vec3::new(0.018, 0.12, 0.018),
            DARK,
            Style::Soft,
        );
    }
}
fn banana(s: &Scene, r: &mut Renderer, p: Vec3, time: f32) {
    for k in 0..5 {
        let a = -0.9 + k as f32 * 0.45;
        let local = Vec3::new(a.sin() * 0.45, a.cos() * -0.45 + 0.4, 0.);
        s.ball(
            r,
            p + local.rotate_y(time),
            Vec3::new(0.14, 0.16, 0.12),
            0.,
            if k == 0 || k == 4 {
                [156, 111, 36]
            } else {
                YELLOW
            },
            Style::Soft,
        );
    }
}
pub fn render(s: &Scene, r: &mut Renderer, g: &Game) {
    r.begin(g.camera_eye, g.camera_target, g.time);
    for y in 0..r.height {
        let t = y as f32 / r.height as f32;
        r.rect(
            0,
            y as i32,
            r.width as i32,
            1,
            [
                (81. + t * 89.) as u8,
                (171. + t * 40.) as u8,
                (211. + t * 14.) as u8,
            ],
        );
    }
    let run = &g.runner;
    // The railway and blocks recycle around the player, preserving bounded geometry.
    s.world.block(
        r,
        Vec3::new(0., -0.35, -45.),
        Vec3::new(50., 0.25, 85.),
        [88, 212, 25],
    );
    s.world.block(
        r,
        Vec3::new(0., -0.08, -40.),
        Vec3::new(4.65, 0.08, 75.),
        [32, 141, 175],
    );
    for side in [-1., 1.] {
        s.world.block(
            r,
            Vec3::new(side * 5.1, 0.12, -40.),
            Vec3::new(0.45, 0.12, 75.),
            [229, 214, 172],
        );
        s.world.block(
            r,
            Vec3::new(side * 5.5, 0.23, -40.),
            Vec3::new(0.08, 0.12, 75.),
            [239, 182, 66],
        );
    }
    for side in [-1., 1.] {
        for k in 0..8 {
            let z = 14. - k as f32 * 16. + run.distance.rem_euclid(16.);
            s.palm(r, Vec3::new(side * 6.5, 0., z), 1.7);
            s.world
                .checker_block(r, Vec3::new(side * 6.5, -0.7, z), Vec3::new(0.8, 0.7, 2.));
        }
    }
    s.hill_loop(
        r,
        Vec3::new(-13., 0., -45. + run.distance.rem_euclid(20.)),
        3.,
    );
    for lane in -1..=1 {
        for side in [-0.93, 0.93] {
            s.world.block(
                r,
                Vec3::new(lane as f32 * 3. + side, 0.035, -40.),
                Vec3::new(0.055, 0.04, 75.),
                [181, 192, 196],
            );
        }
    }
    for k in 0..30 {
        let z = 16. - k as f32 * 4. + run.distance.rem_euclid(4.);
        for lane in -1..=1 {
            s.world.block(
                r,
                Vec3::new(lane as f32 * 3., 0.01, z),
                Vec3::new(1.05, 0.015, 0.13),
                [73, 83, 90],
            );
        }
    }
    for k in 0..10 {
        let z = 16. - k as f32 * 14. + run.distance.rem_euclid(14.);
        for side in [-1., 1.] {
            let x = side * (9.5 + (k % 2) as f32);
            let height = 3. + (k % 4) as f32 * 1.3;
            let color = match (k + if side < 0. { 1 } else { 0 }) % 4 {
                0 => [225, 153, 107],
                1 => [111, 177, 174],
                2 => [216, 189, 105],
                _ => [170, 150, 190],
            };
            s.world.block(
                r,
                Vec3::new(x, height * 0.5, z),
                Vec3::new(2.6, height * 0.5, 4.3),
                color,
            );
            s.world.block(
                r,
                Vec3::new(x, height + 0.1, z),
                Vec3::new(2.85, 0.14, 4.55),
                [241, 223, 186],
            );
            // Bold readable windows rather than a dense screen-space pattern.
            for floor in 0..3 {
                for column in [-1., 1.] {
                    s.world.block(
                        r,
                        Vec3::new(x + column * 1.15, 1.1 + floor as f32 * 1.2, z + 4.32),
                        Vec3::new(0.50, 0.42, 0.025),
                        [52, 105, 135],
                    );
                }
            }
            s.world.block(
                r,
                Vec3::new(side * 5.9, 1.7, z + 4.),
                Vec3::new(0.07, 1.7, 0.07),
                DARK,
            );
            s.ball(
                r,
                Vec3::new(side * 5.9, 3.4, z + 4.),
                Vec3::new(0.25, 0.25, 0.25),
                0.,
                [251, 220, 148],
                Style::Soft,
            );
        }
    }
    // Repeating overhead station portals sell forward speed and depth.
    for k in 0..3 {
        let z = -10. - k as f32 * 42. + run.distance.rem_euclid(42.);
        for side in [-1., 1.] {
            s.world.block(
                r,
                Vec3::new(side * 5.5, 3.4, z),
                Vec3::new(0.19, 3.4, 0.19),
                [45, 119, 135],
            );
        }
        s.world.block(
            r,
            Vec3::new(0., 6.5, z),
            Vec3::new(5.7, 0.25, 0.25),
            [45, 119, 135],
        );
        s.world.block(
            r,
            Vec3::new(0., 6.05, z + 0.1),
            Vec3::new(1.9, 0.42, 0.12),
            [37, 68, 89],
        );
        if z < 0.
            && let Some((x, y, _)) = r.project(Vec3::new(0., 6.05, z + 0.24))
        {
            crate::ui::text(r, x as i32 - 33, y as i32 - 3, "BANANA RUSH", 1, YELLOW);
        }
    }
    for item in &run.items {
        if item.resolved && item.kind == Kind::Coin {
            continue;
        }
        let z = PLAYER_Z - (item.at - run.distance);
        let x = item.lane as f32 * 3.;
        match item.kind {
            Kind::Coin => banana(
                s,
                r,
                Vec3::new(x, 1.0 + (g.time * 4. + item.at).sin() * 0.12, z),
                g.time * 1.5,
            ),
            Kind::Train => {
                let color = if item.lane == 0 {
                    [232, 129, 70]
                } else {
                    [54, 144, 172]
                };
                s.world.block(
                    r,
                    Vec3::new(x, 1.75, z - 3.),
                    Vec3::new(1.14, 1.60, 3.),
                    color,
                );
                s.world.block(
                    r,
                    Vec3::new(x, 3.42, z - 3.),
                    Vec3::new(1.18, 0.12, 3.04),
                    [226, 216, 186],
                );
                s.world.block(
                    r,
                    Vec3::new(x, 2.35, z + 0.02),
                    Vec3::new(0.86, 0.62, 0.04),
                    [41, 75, 102],
                );
                s.world.block(
                    r,
                    Vec3::new(x, 1.15, z + 0.03),
                    Vec3::new(1.12, 0.13, 0.05),
                    [238, 209, 119],
                );
                for side in [-1., 1.] {
                    s.ball(
                        r,
                        Vec3::new(x + side * 0.77, 0.8, z + 0.07),
                        Vec3::new(0.16, 0.16, 0.06),
                        0.,
                        [255, 228, 155],
                        Style::Glow,
                    );
                    for at in [-1.5, -4.5] {
                        s.world.block(
                            r,
                            Vec3::new(x + side * 1.16, 0.35, z + at),
                            Vec3::new(0.10, 0.23, 0.40),
                            DARK,
                        );
                    }
                }
            }
            Kind::Barrier => {
                s.world.block(
                    r,
                    Vec3::new(x, 0.62, z),
                    Vec3::new(1.12, 0.55, 0.25),
                    [223, 82, 65],
                );
                for k in -2..=2 {
                    s.world.block(
                        r,
                        Vec3::new(x + k as f32 * 0.43, 0.64, z + 0.26),
                        Vec3::new(0.12, 0.46, 0.015),
                        [255, 226, 145],
                    );
                }
            }
            Kind::Beam => {
                for side in [-1., 1.] {
                    s.world.block(
                        r,
                        Vec3::new(x + side * 1.18, 1.40, z),
                        Vec3::new(0.10, 1.40, 0.13),
                        [51, 117, 144],
                    );
                }
                s.world.block(
                    r,
                    Vec3::new(x, 2.1, z),
                    Vec3::new(1.25, 0.65, 0.20),
                    [58, 143, 173],
                );
                s.world.block(
                    r,
                    Vec3::new(x, 1.55, z + 0.22),
                    Vec3::new(1.12, 0.12, 0.02),
                    YELLOW,
                );
            }
        }
    }
    s.ball(
        r,
        Vec3::new(run.x, 0.03, PLAYER_Z),
        Vec3::new(0.73, 0.025, 0.58),
        0.,
        [60, 78, 88],
        Style::Glow,
    );
    if run.invulnerable <= 0. || (g.time * 16.) as i32 % 2 == 0 {
        let bounce = if run.slide > 0. {
            0.
        } else {
            (run.distance * 1.5).sin().abs() * 0.06
        };
        minion(
            s,
            r,
            Vec3::new(run.x, run.height() + bounce, PLAYER_Z),
            1.,
            -0.25,
            g.time,
            run.slide > 0.,
        );
    }
    if run.coin_flash > 0. {
        for k in 0..6 {
            let a = k as f32 * TAU / 6. + g.time;
            s.ball(
                r,
                Vec3::new(run.x + a.cos(), 1.2 + a.sin(), PLAYER_Z + 0.5),
                Vec3::new(0.07, 0.07, 0.07),
                0.,
                YELLOW,
                Style::Glow,
            );
        }
    }
}
