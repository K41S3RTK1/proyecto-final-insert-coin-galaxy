use crate::{
    duel::{Attack, Stage},
    game::Game,
    math::Vec3,
    renderer::{Material, Mesh, Renderer, Style},
    scene::Scene,
};
use std::f32::consts::TAU;
fn link(r: &mut Renderer, a: Vec3, b: Vec3, radius: f32, color: [u8; 3]) {
    let center = (a + b) * 0.5;
    let axis = (b - a).normalized();
    let right = if axis.y.abs() < 0.9 {
        Vec3::new(0., 1., 0.).cross(axis).normalized()
    } else {
        Vec3::new(0., 0., 1.).cross(axis).normalized()
    };
    let front = axis.cross(right);
    let half = (b - a).length() * 0.5;
    let mut m = Mesh {
        radius: half + radius,
        vertices: vec![],
        normals: vec![],
        faces: vec![],
    };
    for side in [-1., 1.] {
        for k in 0..=10 {
            let angle = k as f32 * TAU / 10.;
            let n = right * angle.cos() + front * angle.sin();
            m.vertices.push(axis * (half * side) + n * radius);
            m.normals.push(n);
        }
    }
    for k in 0..10 {
        m.faces.push([k, k + 1, k + 11]);
        m.faces.push([k + 1, k + 12, k + 11]);
    }
    r.mesh(
        &m,
        center,
        Vec3::new(1., 1., 1.),
        0.,
        Material::new(color, Style::Ink),
    );
}
fn fighter(s: &Scene, r: &mut Renderer, g: &Game, enemy: bool) {
    let d = &g.duel;
    let side = if enemy { -1. } else { 1. };
    let x = if enemy { d.enemy_x } else { d.player_x };
    let strike = enemy && d.stage == Stage::Strike;
    let telegraph = enemy && d.stage == Stage::Telegraph;
    let punching = if enemy {
        strike && d.attack != Attack::Sweep
    } else {
        d.attack_pose > 0.
    };
    let crouch = if !enemy && d.dodge > 0. { 0.55 } else { 0. };
    let sway = (g.time * 3.).sin() * 0.025;
    let root = Vec3::new(x, -0.95, 3.2);
    let skin = if enemy {
        [201, 143, 107]
    } else {
        [224, 176, 136]
    };
    let uniform = if enemy {
        [152, 77, 113]
    } else {
        [203, 215, 208]
    };
    let belt = if enemy {
        [239, 174, 78]
    } else {
        [75, 149, 173]
    };
    let p = |v: Vec3| root + Vec3::new(v.x * side, v.y - crouch + sway, v.z);
    let hip = p(Vec3::new(0., 1.0, 0.));
    let chest = p(Vec3::new(if punching { 0.18 } else { 0. }, 1.78, 0.));
    // Shadow, split trousers, wrap jacket and belt.
    s.ball(
        r,
        root + Vec3::new(0., 0.025, 0.),
        Vec3::new(0.7, 0.025, 0.42),
        0.,
        [32, 30, 48],
        Style::Glow,
    );
    s.ball(
        r,
        chest,
        Vec3::new(0.45, 0.59, 0.28),
        0.,
        uniform,
        Style::Ink,
    );
    s.ball(r, hip, Vec3::new(0.40, 0.16, 0.30), 0., belt, Style::Ink);
    let kick = enemy && strike && d.attack == Attack::Sweep;
    for leg in [-1., 1.] {
        let top = hip + Vec3::new(leg * 0.22, 0., 0.);
        let knee = p(Vec3::new(leg * 0.35, 0.52, 0.06));
        let foot = if kick && leg < 0. {
            p(Vec3::new(1.45, 0.48, 0.08))
        } else {
            root + Vec3::new(leg * 0.48, 0.12, 0.10)
        };
        link(r, top, knee, 0.20, uniform);
        link(r, knee, foot, 0.17, uniform);
        s.ball(
            r,
            foot,
            Vec3::new(0.30, 0.13, 0.24),
            0.,
            [67, 61, 75],
            Style::Ink,
        );
    }
    let guarding = !enemy && d.blocking;
    for arm in [-1., 1.] {
        let shoulder = chest + Vec3::new(arm * 0.38, 0.22, 0.);
        let elbow = if guarding {
            p(Vec3::new(0.45, 1.97, 0.30))
        } else {
            p(Vec3::new(arm * 0.55, 1.45, 0.2))
        };
        let fist = if punching && arm > 0. {
            p(Vec3::new(1.6, 1.92, 0.25))
        } else if guarding {
            p(Vec3::new(0.55, 2.42 + arm * 0.14, 0.34))
        } else if telegraph && d.attack == Attack::Jab && arm > 0. {
            p(Vec3::new(0.1, 2.35, 0.25))
        } else if enemy && d.attack == Attack::Wave && (telegraph || strike) {
            p(Vec3::new(0.60, 1.60, 0.4 + arm * 0.18))
        } else {
            p(Vec3::new(0.42 + arm * 0.20, 1.92, 0.40))
        };
        link(r, shoulder, elbow, 0.18, uniform);
        link(r, elbow, fist, 0.13, skin);
        s.ball(r, fist, Vec3::new(0.18, 0.18, 0.16), 0., belt, Style::Ink);
    }
    let head = p(Vec3::new(0.05, 2.66, 0.));
    s.ball(r, head, Vec3::new(0.34, 0.41, 0.31), 0., skin, Style::Ink);
    s.ball(
        r,
        head + Vec3::new(0., 0.29, -0.02),
        Vec3::new(0.35, 0.16, 0.31),
        0.,
        [45, 38, 47],
        Style::Ink,
    );
    s.ball(
        r,
        head + Vec3::new(0., 0.13, 0.02),
        Vec3::new(0.36, 0.055, 0.32),
        0.,
        belt,
        Style::Ink,
    );
    for eye in [-1., 1.] {
        s.ball(
            r,
            head + Vec3::new(eye * 0.12, 0.04, 0.29),
            Vec3::new(0.045, 0.045, 0.025),
            0.,
            [36, 32, 42],
            Style::Glow,
        );
    }
    if enemy && d.attack == Attack::Wave && (telegraph || strike) {
        s.ball(
            r,
            p(Vec3::new(if strike { 1.6 } else { 0.85 }, 1.65, 0.42)),
            Vec3::new(0.32, 0.32, 0.32),
            0.,
            [92, 222, 240],
            Style::Glow,
        );
    }
    if (enemy && d.counter_open) || (!enemy && d.dodge > 0.) {
        for k in 0..16 {
            let a = k as f32 * TAU / 16.;
            s.ball(
                r,
                root + Vec3::new(a.cos() * 0.8, 0.06, a.sin() * 0.6),
                Vec3::new(0.035, 0.035, 0.035),
                0.,
                [108, 220, 209],
                Style::Glow,
            );
        }
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
                (31. + t * 79.) as u8,
                (72. + t * 125.) as u8,
                (185. + t * 65.) as u8,
            ],
        );
    }
    for side in [-1., 1.] {
        s.palm(r, Vec3::new(side * 8., -1., -1.), 2.2);
        s.hill_loop(r, Vec3::new(side * 13., -1., -8.), 2.5);
    }
    // Distinct urban skyline and illuminated signs.
    for k in 0..18 {
        let x = -18. + k as f32 * 2.2;
        let height = 3. + (k * 7 % 6) as f32;
        let p = Vec3::new(x, height * 0.5 - 1., -11. - (k % 3) as f32);
        let mesh = s.world.surface_block(
            Vec3::default(),
            Vec3::new(0.8, height * 0.5, 1.0),
            Vec3::new(0., 1., 0.),
        );
        r.mesh(
            &mesh,
            p,
            Vec3::new(1., 1., 1.),
            0.,
            Material::new([42, 45, 72], Style::City),
        );
    }
    s.world.block(
        r,
        Vec3::new(0., -1.2, 0.),
        Vec3::new(13., 0.2, 13.),
        [75, 66, 87],
    );
    for x in -6_i32..=6 {
        for z in -3_i32..=5 {
            s.world.block(
                r,
                Vec3::new(x as f32 * 1.5, -0.98, z as f32 * 1.5),
                Vec3::new(0.73, 0.025, 0.73),
                if (x + z) % 2 == 0 {
                    [126, 52, 16]
                } else {
                    [205, 111, 37]
                },
            );
        }
    }
    // Raised dojo, veranda, pillars, layered roof and lanterns.
    s.world.block(
        r,
        Vec3::new(0., -0.6, -4.),
        Vec3::new(5.6, 0.4, 2.0),
        [92, 63, 70],
    );
    s.world.block(
        r,
        Vec3::new(0., 1.6, -5.6),
        Vec3::new(5.1, 2.2, 0.15),
        [196, 151, 133],
    );
    for x in [-4.8, -2.8, 2.8, 4.8] {
        s.world.block(
            r,
            Vec3::new(x, 1.4, -3.5),
            Vec3::new(0.16, 2.3, 0.16),
            [140, 63, 70],
        );
    }
    for tier in 0..3 {
        let y = 4.1 + tier as f32 * 0.6;
        let width = 6.1 - tier as f32 * 0.55;
        s.world.block(
            r,
            Vec3::new(0., y, -4.7),
            Vec3::new(width, 0.18, 2.5 - tier as f32 * 0.3),
            [59, 65, 83],
        );
        s.world.block(
            r,
            Vec3::new(0., y + 0.19, -2.2 - tier as f32 * 0.3),
            Vec3::new(width, 0.035, 0.035),
            [205, 118, 125],
        );
    }
    for x in [-3.8, -1.8, 1.8, 3.8] {
        s.ball(
            r,
            Vec3::new(x, 3., -3.2),
            Vec3::new(0.30, 0.45, 0.30),
            0.,
            [241, 119, 94],
            Style::Ink,
        );
        s.world.block(
            r,
            Vec3::new(x, 3.65, -3.2),
            Vec3::new(0.025, 0.25, 0.025),
            [100, 80, 83],
        );
    }
    s.world.block(
        r,
        Vec3::new(0., 2.2, -5.35),
        Vec3::new(1.2, 1.3, 0.05),
        [92, 79, 99],
    );
    s.world.block(
        r,
        Vec3::new(0., 3.35, -3.42),
        Vec3::new(1.8, 0.38, 0.06),
        [37, 38, 59],
    );
    for side in [-1., 1.] {
        s.world.block(
            r,
            Vec3::new(side * 7., 0.8, -1.),
            Vec3::new(0.65, 1.8, 0.5),
            [48, 43, 70],
        );
        for k in 0..4 {
            s.world.block(
                r,
                Vec3::new(side * 7., 0.1 + k as f32 * 0.45, -0.48),
                Vec3::new(0.48, 0.035, 0.02),
                [79, 205, 209],
            );
        }
    }
    fighter(s, r, g, false);
    fighter(s, r, g, true);
    if g.duel.hit_flash > 0. {
        let x = if g.duel.message == "GOLPE RECIBIDO" {
            g.duel.player_x
        } else {
            g.duel.enemy_x
        };
        for k in 0..8 {
            let a = k as f32 * TAU / 8.;
            s.ball(
                r,
                Vec3::new(x + a.cos() * 0.6, 1. + a.sin() * 0.6, 3.65),
                Vec3::new(0.06, 0.06, 0.06),
                0.,
                [255, 222, 139],
                Style::Glow,
            );
        }
    }
    if let Some((x, y, _)) = r.project(Vec3::new(0., 3.4, -3.3)) {
        let size = (r.width / 400).max(1) as i32;
        crate::ui::text(
            r,
            x as i32 - 9 * 3 * size,
            y as i32 - 3 * size,
            "NEON DOJO",
            size,
            [235, 184, 140],
        );
    }
}
