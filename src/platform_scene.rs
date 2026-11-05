//! Authored 2.5D Mushroom Kingdom: solid geometry, orthographic camera and parallax.
use crate::{
    game::Game,
    math::Vec3,
    platformer::{BlockKind, FLAG_X, TILE},
    renderer::{Material, Mesh, Renderer, Style},
    scene::Scene,
};
const RED: [u8; 3] = [232, 43, 39];
const BLUE: [u8; 3] = [33, 94, 205];
const SKIN: [u8; 3] = [255, 188, 132];
const WHITE: [u8; 3] = [255, 249, 226];
fn cube(s: &Scene, r: &mut Renderer, p: Vec3, scale: Vec3, c: [u8; 3]) {
    s.world.block(r, p, scale, c);
}
fn ball(s: &Scene, r: &mut Renderer, p: Vec3, scale: Vec3, c: [u8; 3]) {
    s.ball(r, p, scale, 0., c, Style::Soft);
}
fn tube(s: &Scene, r: &mut Renderer, p: Vec3, scale: Vec3, c: [u8; 3]) {
    r.mesh(&s.cylinder, p, scale, 0., Material::new(c, Style::Soft));
}
fn brick(s: &Scene, r: &mut Renderer, x: f32, y: f32, z: f32, size: f32, stair: bool) {
    let p = Vec3::new(x, y, z);
    cube(
        s,
        r,
        p,
        Vec3::new(size * 0.49, size * 0.49, 0.52),
        [125, 60, 33],
    );
    if stair {
        cube(
            s,
            r,
            p + Vec3::new(0., 0., 0.035),
            Vec3::new(size * 0.43, size * 0.43, 0.52),
            [207, 132, 66],
        );
        cube(
            s,
            r,
            p + Vec3::new(-size * 0.36, size * 0.33, 0.57),
            Vec3::new(size * 0.03, size * 0.04, 0.015),
            [247, 184, 101],
        );
    } else {
        for row in 0..3 {
            for col in 0..2 {
                let xx = (col as f32 - 0.5) * size * 0.48;
                cube(
                    s,
                    r,
                    p + Vec3::new(xx, (row as f32 - 1.) * size * 0.31, 0.025),
                    Vec3::new(size * 0.226, size * 0.14, 0.52),
                    if row == 2 {
                        [218, 123, 67]
                    } else {
                        [190, 87, 48]
                    },
                );
            }
        }
    }
}
fn question(s: &Scene, r: &mut Renderer, x: f32, y: f32, used: bool) {
    let p = Vec3::new(x, y, 0.);
    cube(
        s,
        r,
        p,
        Vec3::new(0.59, 0.59, 0.58),
        if used { [132, 83, 45] } else { [204, 125, 26] },
    );
    cube(
        s,
        r,
        p + Vec3::new(0., 0., 0.045),
        Vec3::new(0.53, 0.53, 0.55),
        if used { [174, 124, 66] } else { [255, 193, 48] },
    );
    for a in [-0.41, 0.41] {
        for b in [-0.41, 0.41] {
            ball(
                s,
                r,
                p + Vec3::new(a, b, 0.60),
                Vec3::new(0.045, 0.045, 0.025),
                [128, 77, 31],
            );
        }
    }
    if !used {
        // Rounded sculpted question mark, visible as geometry at any resolution.
        for (a, b, w, h) in [
            (0., 0.29, 0.17, 0.06),
            (-0.17, 0.20, 0.06, 0.10),
            (0.17, 0.15, 0.06, 0.15),
            (0.09, 0.015, 0.10, 0.06),
            (0., -0.08, 0.06, 0.12),
            (0., -0.32, 0.07, 0.06),
        ] {
            cube(
                s,
                r,
                p + Vec3::new(a + 0.025, b - 0.025, 0.61),
                Vec3::new(w, h, 0.03),
                [162, 95, 24],
            );
            cube(
                s,
                r,
                p + Vec3::new(a, b, 0.66),
                Vec3::new(w, h, 0.03),
                WHITE,
            );
        }
    }
}
fn pipe(s: &Scene, r: &mut Renderer, x: f32, y: f32, h: f32) {
    tube(
        s,
        r,
        Vec3::new(x, y + h * 0.5, 0.),
        Vec3::new(1.11, h * 0.5, 0.85),
        [37, 168, 55],
    );
    tube(
        s,
        r,
        Vec3::new(x, y + h - 0.27, 0.),
        Vec3::new(1.27, 0.28, 0.98),
        [65, 205, 63],
    );
    tube(
        s,
        r,
        Vec3::new(x, y + h + 0.017, 0.),
        Vec3::new(1.03, 0.018, 0.77),
        [13, 67, 37],
    );
    tube(
        s,
        r,
        Vec3::new(x, y + 0.08, 0.),
        Vec3::new(1.14, 0.08, 0.88),
        [22, 121, 47],
    );
}
#[allow(clippy::too_many_arguments)]
pub fn mario(
    s: &Scene,
    r: &mut Renderer,
    p: Vec3,
    scale: f32,
    facing: f32,
    time: f32,
    moving: bool,
    jumping: bool,
) {
    let yaw = facing * 0.37;
    let phase = if moving { (time * 14.).sin() } else { 0. };
    let mut part = |pos: Vec3, sz: Vec3, c| {
        s.ball(
            r,
            p + pos.rotate_y(yaw) * scale,
            sz * scale,
            yaw,
            c,
            Style::Soft,
        )
    };
    // Feet, articulated legs, overalls and shoulder straps.
    for side in [-1., 1.] {
        let lift = if jumping {
            0.10
        } else {
            (phase * side).max(0.) * 0.10
        };
        part(
            Vec3::new(side * 0.17, 0.13 + lift, 0.09 + phase * side * 0.10),
            Vec3::new(0.20, 0.12, 0.29),
            [108, 54, 30],
        );
        part(
            Vec3::new(side * 0.16, 0.35 + lift, phase * side * 0.08),
            Vec3::new(0.15, 0.25, 0.16),
            BLUE,
        );
    }
    part(Vec3::new(0., 0.73, 0.), Vec3::new(0.36, 0.35, 0.25), RED);
    part(Vec3::new(0., 0.54, 0.06), Vec3::new(0.34, 0.25, 0.25), BLUE);
    part(Vec3::new(0., 0.73, 0.22), Vec3::new(0.19, 0.22, 0.07), BLUE);
    for side in [-1., 1.] {
        part(
            Vec3::new(side * 0.20, 0.85, 0.17),
            Vec3::new(0.06, 0.22, 0.07),
            BLUE,
        );
        part(
            Vec3::new(side * 0.19, 0.80, 0.27),
            Vec3::new(0.048, 0.05, 0.026),
            [255, 216, 56],
        );
        let arm = if jumping { 0.22 } else { phase * side * 0.13 };
        part(
            Vec3::new(side * 0.40, 0.74 + arm, 0.),
            Vec3::new(0.14, 0.24, 0.14),
            RED,
        );
        part(
            Vec3::new(side * 0.43, 0.52 + arm, 0.10),
            Vec3::new(0.16, 0.15, 0.15),
            WHITE,
        );
        part(
            Vec3::new(side * 0.35, 1.19, 0.),
            Vec3::new(0.10, 0.13, 0.10),
            SKIN,
        );
    }
    part(
        Vec3::new(0., 1.21, 0.),
        Vec3::new(0.35, 0.33, 0.29),
        [101, 48, 26],
    );
    part(Vec3::new(0., 1.20, 0.12), Vec3::new(0.32, 0.29, 0.26), SKIN);
    for side in [-1., 1.] {
        part(
            Vec3::new(side * 0.13, 1.29, 0.33),
            Vec3::new(0.087, 0.12, 0.035),
            WHITE,
        );
        part(
            Vec3::new(side * 0.12, 1.29, 0.364),
            Vec3::new(0.046, 0.078, 0.025),
            [35, 116, 201],
        );
        part(
            Vec3::new(side * 0.12, 1.29, 0.383),
            Vec3::new(0.025, 0.056, 0.012),
            [20, 29, 38],
        );
        part(
            Vec3::new(side * 0.11, 1.32, 0.395),
            Vec3::new(0.012, 0.018, 0.006),
            WHITE,
        );
        part(
            Vec3::new(side * 0.13, 1.42, 0.31),
            Vec3::new(0.105, 0.033, 0.035),
            [79, 39, 23],
        );
    }
    // Scalloped moustache and signature round nose.
    for k in -2..=2 {
        let x = k as f32 * 0.075;
        part(
            Vec3::new(x, 1.08 + x.abs() * 0.26, 0.355),
            Vec3::new(0.075, 0.065, 0.055),
            [62, 31, 21],
        );
    }
    part(
        Vec3::new(0., 1.18, 0.40),
        Vec3::new(0.135, 0.11, 0.12),
        SKIN,
    );
    part(Vec3::new(0., 1.51, 0.015), Vec3::new(0.37, 0.19, 0.31), RED);
    part(
        Vec3::new(0., 1.44, 0.27),
        Vec3::new(0.38, 0.045, 0.23),
        [209, 29, 31],
    );
    part(
        Vec3::new(0., 1.55, 0.293),
        Vec3::new(0.12, 0.10, 0.024),
        WHITE,
    );
    // Red M on the cap, made from small geometry rather than a low-resolution sprite.
    for (x, y) in [
        (-0.065, 1.54),
        (-0.065, 1.58),
        (-0.035, 1.57),
        (0., 1.54),
        (0.035, 1.57),
        (0.065, 1.58),
        (0.065, 1.54),
    ] {
        part(Vec3::new(x, y, 0.32), Vec3::new(0.021, 0.024, 0.012), RED);
    }
}
fn goomba(s: &Scene, r: &mut Renderer, x: f32, y: f32, time: f32, squash: bool) {
    let q = if squash { 0.23 } else { 1. };
    let p = Vec3::new(x, y, 0.);
    ball(
        s,
        r,
        p + Vec3::new(0., 0.26 * q, 0.),
        Vec3::new(0.31, 0.30 * q, 0.29),
        [236, 188, 125],
    );
    ball(
        s,
        r,
        p + Vec3::new(0., 0.66 * q, 0.),
        Vec3::new(0.57, 0.38 * q, 0.34),
        [157, 79, 38],
    );
    for side in [-1., 1.] {
        ball(
            s,
            r,
            p + Vec3::new(side * 0.29, 0.11, 0.13 + (time * 9.).sin() * side * 0.06),
            Vec3::new(0.25, 0.12, 0.28),
            [78, 42, 29],
        );
        ball(
            s,
            r,
            p + Vec3::new(side * 0.18, 0.66 * q, 0.30),
            Vec3::new(0.12, 0.17 * q, 0.06),
            WHITE,
        );
        ball(
            s,
            r,
            p + Vec3::new(side * 0.15, 0.65 * q, 0.355),
            Vec3::new(0.045, 0.10 * q, 0.022),
            [30, 29, 31],
        );
        ball(
            s,
            r,
            p + Vec3::new(side * 0.17, 0.81 * q, 0.31),
            Vec3::new(0.18, 0.045 * q, 0.045),
            [56, 33, 26],
        );
    }
}
fn coin(s: &Scene, r: &mut Renderer, p: Vec3, time: f32) {
    let a = time * 2.;
    s.ball(
        r,
        p,
        Vec3::new(0.31, 0.43, 0.09),
        a,
        [255, 199, 36],
        Style::Metal,
    );
    s.ball(
        r,
        p + Vec3::new(0., 0., 0.086).rotate_y(a),
        Vec3::new(0.19, 0.31, 0.025),
        a,
        [255, 232, 102],
        Style::Metal,
    );
}
fn mushroom(s: &Scene, r: &mut Renderer, p: Vec3) {
    ball(
        s,
        r,
        p + Vec3::new(0., 0.30, 0.),
        Vec3::new(0.28, 0.31, 0.26),
        WHITE,
    );
    ball(
        s,
        r,
        p + Vec3::new(0., 0.65, 0.),
        Vec3::new(0.55, 0.33, 0.42),
        RED,
    );
    for x in [-0.32, 0., 0.32] {
        ball(
            s,
            r,
            p + Vec3::new(x, 0.72, 0.30),
            Vec3::new(0.13, 0.12, 0.08),
            WHITE,
        );
    }
    for x in [-0.10, 0.10] {
        ball(
            s,
            r,
            p + Vec3::new(x, 0.33, 0.25),
            Vec3::new(0.036, 0.09, 0.025),
            [39, 37, 40],
        );
    }
}
fn castle(s: &Scene, r: &mut Renderer) {
    let x = 205. * TILE;
    cube(
        s,
        r,
        Vec3::new(x, 2., -0.6),
        Vec3::new(3., 2., 1.4),
        [171, 91, 54],
    );
    for row in 0..6 {
        for col in 0..6 {
            brick(
                s,
                r,
                x - 2.5 + col as f32,
                row as f32 * 0.65 + 0.33,
                0.35,
                0.65,
                false,
            );
        }
    }
    for side in [-1., 1.] {
        cube(
            s,
            r,
            Vec3::new(x + side * 2.5, 3.1, 0.),
            Vec3::new(0.78, 3.1, 1.1),
            [190, 105, 60],
        );
        for k in -1..=1 {
            cube(
                s,
                r,
                Vec3::new(x + side * 2.5 + k as f32 * 0.53, 6.4, 0.),
                Vec3::new(0.20, 0.35, 1.1),
                [224, 143, 83],
            );
        }
        ball(
            s,
            r,
            Vec3::new(x + side * 2.5, 4.3, 1.12),
            Vec3::new(0.25, 0.44, 0.03),
            [57, 42, 35],
        );
    }
    for k in -2..=2 {
        cube(
            s,
            r,
            Vec3::new(x + k as f32 * 0.7, 4.3, -0.6),
            Vec3::new(0.22, 0.35, 1.4),
            [218, 140, 83],
        );
    }
    ball(
        s,
        r,
        Vec3::new(x, 1.10, 0.85),
        Vec3::new(0.75, 1.35, 0.035),
        [51, 38, 30],
    );
    cube(
        s,
        r,
        Vec3::new(x, 0.5, 0.85),
        Vec3::new(0.74, 0.5, 0.04),
        [51, 38, 30],
    );
}
pub fn render(s: &Scene, r: &mut Renderer, g: &Game) {
    let p = &g.platformer;
    let cam = p.camera;
    let cy = 5.2 + (p.y - 4.).max(0.) * 0.55;
    r.begin_ortho(
        Vec3::new(cam, cy + 1.1, 22.),
        Vec3::new(cam, cy, 0.),
        g.time,
        16.,
    );
    // Clear the space background. Gradient and scenery belong to this world only.
    for y in 0..r.height {
        let t = y as f32 / r.height as f32;
        r.rect(
            0,
            y as i32,
            r.width as i32,
            1,
            [
                (63. + t * 87.) as u8,
                (164. + t * 57.) as u8,
                (237. + t * 6.) as u8,
            ],
        );
    }
    for k in -3..10 {
        let x = k as f32 * 12. + cam * 0.48;
        let height = 3.8 + (k as f32 * 2.1).sin().abs() * 3.2;
        ball(
            s,
            r,
            Vec3::new(x, 0., -10.),
            Vec3::new(4., height, 2.),
            [68, 207, 37],
        );
        for dx in [-0.8, 0.8] {
            ball(
                s,
                r,
                Vec3::new(x + dx, 1.6, -7.9),
                Vec3::new(0.10, 0.38, 0.04),
                [53, 142, 84],
            );
        }
    }
    for k in -2..10 {
        let x = k as f32 * 14. + cam * 0.22;
        let y = 9.5 + (k as f32 * 1.8).sin() * 1.6;
        for (dx, dy, sx, sy) in [
            (-1.1, 0., 1.1, 0.54),
            (0., 0.35, 1.05, 0.80),
            (1.1, 0., 1.25, 0.54),
        ] {
            ball(
                s,
                r,
                Vec3::new(x + dx, y + dy, -8.),
                Vec3::new(sx, sy, 0.65),
                WHITE,
            );
        }
    }
    for k in -2..5 {
        let x = (cam * 0.65 / 15.).floor() * 15. + k as f32 * 15.;
        s.palm(r, Vec3::new(x, 0., -3.5), 2.3);
        if k % 3 == 0 {
            s.hill_loop(r, Vec3::new(x + 5., 0., -5.), 2.);
        }
    }
    let left = cam - 20.;
    let right = cam + 20.;
    for b in &p.blocks {
        if b.broken || b.x > right || b.x + b.w < left {
            continue;
        }
        match b.kind {
            BlockKind::Ground => {
                cube(
                    s,
                    r,
                    Vec3::new(b.x + b.w / 2., -1.55, -0.15),
                    Vec3::new(b.w / 2., 1.55, 1.25),
                    [138, 83, 46],
                );
                let start = ((left - b.x) / TILE).floor().max(0.) as i32;
                let end = ((right - b.x) / TILE).ceil().min(b.w / TILE) as i32;
                for k in start..end {
                    let x = b.x + (k as f32 + 0.5) * TILE;
                    for row in 0..3 {
                        cube(
                            s,
                            r,
                            Vec3::new(x, -0.45 - row as f32 * 0.7, 1.13),
                            Vec3::new(0.57, 0.32, 0.05),
                            if (k + row) % 2 == 0 {
                                [209, 118, 40]
                            } else {
                                [125, 51, 14]
                            },
                        );
                    }
                    cube(
                        s,
                        r,
                        Vec3::new(x, -0.055, 0.),
                        Vec3::new(0.60, 0.055, 1.33),
                        [105, 232, 18],
                    );
                    cube(
                        s,
                        r,
                        Vec3::new(x, -0.14, 1.24),
                        Vec3::new(0.60, 0.08, 0.10),
                        [50, 143, 52],
                    );
                }
            }
            BlockKind::Pipe => pipe(s, r, b.x + b.w * 0.5, b.y, b.h),
            BlockKind::Stair => {
                for row in 0..(b.h / TILE).round() as i32 {
                    brick(
                        s,
                        r,
                        b.x + TILE / 2.,
                        b.y + (row as f32 + 0.5) * TILE,
                        0.,
                        TILE,
                        true,
                    );
                }
            }
            BlockKind::Brick => brick(
                s,
                r,
                b.x + TILE / 2.,
                b.y + TILE / 2. + (b.bump / 0.22 * std::f32::consts::PI).sin() * 0.18,
                0.,
                TILE,
                false,
            ),
            BlockKind::Question | BlockKind::Used => question(
                s,
                r,
                b.x + TILE / 2.,
                b.y + TILE / 2. + (b.bump / 0.22 * std::f32::consts::PI).sin() * 0.18,
                b.kind == BlockKind::Used,
            ),
        }
    }
    for c in &p.pickups {
        if !c.taken && c.x > left && c.x < right {
            coin(s, r, Vec3::new(c.x, c.y, 0.), p.time + c.x);
        }
    }
    for e in &p.enemies {
        if (e.alive || e.squash > 0.) && e.x > left && e.x < right {
            goomba(s, r, e.x, e.y, p.time, !e.alive);
        }
    }
    if let Some(m) = p.mushroom
        && m.active
    {
        mushroom(s, r, Vec3::new(m.x, m.y, 0.));
    }
    if p.invulnerable <= 0. || (p.invulnerable * 12.) as i32 % 2 == 0 {
        ball(
            s,
            r,
            Vec3::new(p.x, 0.025, 0.),
            Vec3::new(0.56 / (1. + p.y * 0.07), 0.022, 0.38),
            [71, 119, 48],
        );
        mario(
            s,
            r,
            Vec3::new(p.x, p.y, 0.10),
            if p.big { 1.27 } else { 1. },
            p.facing,
            p.time,
            p.vx.abs() > 0.4,
            !p.grounded,
        );
    }
    if right > FLAG_X - 2. {
        tube(
            s,
            r,
            Vec3::new(FLAG_X, 5.5, 0.),
            Vec3::new(0.07, 5.5, 0.07),
            [229, 235, 218],
        );
        ball(
            s,
            r,
            Vec3::new(FLAG_X, 11.1, 0.),
            Vec3::new(0.19, 0.19, 0.19),
            [255, 218, 68],
        );
        let y = if p.finished { 2. } else { 10. };
        let mut flag = Mesh {
            radius: 2.,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
        };
        for k in 0..=16 {
            let t = k as f32 / 16.;
            let wave = (p.time * 3. - t * 4.).sin() * 0.09 * t;
            for side in [-1., 1.] {
                flag.vertices
                    .push(Vec3::new(-t * 1.6, side * (1. - t) * 0.72, wave));
                flag.normals.push(Vec3::new(0., 0., 1.));
            }
        }
        for k in 0..16 {
            let j = k * 2;
            flag.faces
                .extend([[j, j + 1, j + 2], [j + 2, j + 1, j + 3]]);
        }
        r.mesh(
            &flag,
            Vec3::new(FLAG_X - 0.08, y, 0.),
            Vec3::new(1., 1., 1.),
            0.,
            Material::new([53, 181, 75], Style::Soft),
        );
        castle(s, r);
    }
}
