//! The optional challenge is an authored planet interior. Its coordinates are
//! stable: neither camera nor scenery inherits the planet's orbital rotation.
use crate::{
    game::Game,
    math::Vec3,
    renderer::{Material, Mesh, Renderer, Style},
    scene::Scene,
};
use std::f32::consts::TAU;
pub struct World {
    ground: Mesh,
    cube: Mesh,
    trunk: Mesh,
}
impl World {
    pub fn new() -> Self {
        let mut ground = Mesh {
            radius: 75.,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
        };
        for z in 0..=24 {
            for x in 0..=24 {
                let xx = x as f32 * 3. - 36.;
                let zz = z as f32 * 3. - 48.;
                let y = -3.5 + (xx * 0.12).sin() * (zz * 0.1).cos() * 0.16;
                ground.vertices.push(Vec3::new(xx, y, zz));
                ground.normals.push(Vec3::new(0., 1., 0.));
            }
        }
        for z in 0..24 {
            for x in 0..24 {
                let a = z * 25 + x;
                ground.faces.push([a, a + 25, a + 1]);
                ground.faces.push([a + 1, a + 25, a + 26]);
            }
        }
        let mut cube = Mesh {
            radius: 1.74,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
        };
        for (normal, up, right) in [
            (
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 0., 1.),
                Vec3::new(1., 0., 0.),
            ),
            (
                Vec3::new(0., -1., 0.),
                Vec3::new(0., 0., 1.),
                Vec3::new(-1., 0., 0.),
            ),
            (
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 1., 0.),
                Vec3::new(1., 0., 0.),
            ),
            (
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 1., 0.),
                Vec3::new(-1., 0., 0.),
            ),
            (
                Vec3::new(1., 0., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 0., -1.),
            ),
            (
                Vec3::new(-1., 0., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 0., 1.),
            ),
        ] {
            let start = cube.vertices.len();
            for (x, y) in [(-1., -1.), (1., -1.), (1., 1.), (-1., 1.)] {
                cube.vertices.push(normal + right * x + up * y);
                cube.normals.push(normal);
            }
            cube.faces.push([start, start + 1, start + 2]);
            cube.faces.push([start, start + 2, start + 3]);
        }
        let mut trunk = Mesh {
            radius: 7.,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
        };
        for ring in 0..=12 {
            let y = ring as f32 * 0.55;
            let bend = (y * 0.65).sin() * 0.65;
            let radius = 0.62 - ring as f32 * 0.025;
            for k in 0..=12 {
                let a = k as f32 * TAU / 12.;
                let n = Vec3::new(a.cos(), 0., a.sin());
                trunk.vertices.push(Vec3::new(bend, y, 0.) + n * radius);
                trunk.normals.push(n);
            }
        }
        for ring in 0..12 {
            for k in 0..12 {
                let a = ring * 13 + k;
                trunk.faces.push([a, a + 1, a + 13]);
                trunk.faces.push([a + 1, a + 14, a + 13]);
            }
        }
        Self {
            ground,
            cube,
            trunk,
        }
    }
    pub(crate) fn block(&self, r: &mut Renderer, p: Vec3, scale: Vec3, color: [u8; 3]) {
        r.mesh(&self.cube, p, scale, 0., Material::new(color, Style::Soft));
    }
    pub(crate) fn checker_block(&self, r: &mut Renderer, p: Vec3, scale: Vec3) {
        r.mesh(
            &self.cube,
            p,
            scale,
            0.,
            Material::new([205, 111, 37], Style::Checker),
        );
    }
    pub(crate) fn surface_block(&self, center: Vec3, scale: Vec3, up: Vec3) -> Mesh {
        let right = if up.y.abs() < 0.9 {
            Vec3::new(0., 1., 0.).cross(up).normalized()
        } else {
            Vec3::new(0., 0., 1.).cross(up).normalized()
        };
        let forward = right.cross(up);
        let vertices: Vec<_> = self
            .cube
            .vertices
            .iter()
            .map(|p| {
                center + right * (p.x * scale.x) + up * (p.y * scale.y) + forward * (p.z * scale.z)
            })
            .collect();
        let normals = self
            .cube
            .normals
            .iter()
            .map(|n| right * n.x + up * n.y + forward * n.z)
            .collect();
        let radius = vertices.iter().map(|v| v.length()).fold(0., f32::max);
        Mesh {
            vertices,
            normals,
            faces: self.cube.faces.clone(),
            radius,
        }
    }
    pub fn render(&self, s: &Scene, r: &mut Renderer, g: &Game) {
        r.begin(g.camera_eye, g.camera_target, g.time);
        // A painted atmospheric sky in the same CPU framebuffer, no space behind it.
        for y in 0..r.height {
            let t = y as f32 / r.height as f32;
            let color = [
                (27. + t * 106.) as u8,
                (111. + t * 117.) as u8,
                (232. + t * 23.) as u8,
            ];
            r.rect(0, y as i32, r.width as i32, 1, color);
        }
        r.mesh(
            &self.ground,
            Vec3::default(),
            Vec3::new(1., 1., 1.),
            0.,
            Material::new([150, 169, 105], Style::Grass),
        );
        for side in [-1., 1.] {
            for k in 0..4 {
                let p = Vec3::new(side * (13. + k as f32), -3.4, 8. - k as f32 * 9.);
                self.checker_block(r, p - Vec3::new(0., 1.5, 0.), Vec3::new(3., 1.5, 3.));
                s.palm(r, p, 2.4);
            }
        }
        s.hill_loop(r, Vec3::new(-13., -3.3, -16.), 3.);
        // Layered distant hills; their depth and parallax come from real meshes.
        for (x, y, z, sx, sy, color) in [
            (-18., -3., -15., 10., 6., [70, 197, 27]),
            (17., -3., -16., 11., 7., [54, 181, 22]),
            (29., -3., -28., 17., 11., [32, 166, 76]),
            (-25., -3., -30., 16., 12., [58, 187, 64]),
            (-9., -3., -33., 16., 10., [78, 209, 80]),
            (12., -3., -35., 19., 14., [46, 190, 80]),
        ] {
            s.ball(
                r,
                Vec3::new(x, y, z),
                Vec3::new(sx, sy, 8.),
                0.,
                color,
                Style::Soft,
            );
        }
        for (x, y, z, scale) in [
            (-19., 13., -26., 2.),
            (0., 16., -32., 2.3),
            (24., 12., -28., 1.8),
            (-5., 10., -18., 1.3),
        ] {
            for k in 0..5 {
                let xx = x + (k as f32 - 2.) * scale * 1.2;
                let yy = y + if k == 2 { scale * 0.45 } else { 0. };
                s.ball(
                    r,
                    Vec3::new(xx, yy, z),
                    Vec3::new(scale * 1.5, scale * 0.65, scale * 0.7),
                    0.,
                    [246, 253, 255],
                    Style::Soft,
                );
            }
        }
        // Path of hand-shaped stepping stones leads into the little garden.
        for k in 0..12 {
            let z = 10. - k as f32 * 2.;
            let x = (k as f32 * 0.65).sin() * 1.5;
            s.ball(
                r,
                Vec3::new(x, -3.30, z),
                Vec3::new(1.15, 0.10, 0.65),
                0.,
                [201, 190, 145],
                Style::Soft,
            );
        }
        for (x, z, scale) in [
            (-10., 4., 1.15),
            (10., 1., 1.3),
            (-13., -9., 1.1),
            (13., -12., 1.0),
        ] {
            let p = Vec3::new(x, -3.4, z);
            s.ball(
                r,
                p + Vec3::new(0.6, 0.06, 0.),
                Vec3::new(3.0, 0.045, 2.0),
                0.,
                [91, 119, 78],
                Style::Glow,
            );
            r.mesh(
                &self.trunk,
                p,
                Vec3::new(scale, scale, scale),
                x * 0.15,
                Material::new([135, 102, 69], Style::Ink),
            );
            for k in 0..5 {
                let a = k as f32 * TAU / 5.;
                s.ball(
                    r,
                    p + Vec3::new(a.cos() * 1.7, 6.5 + a.sin() * 0.5, a.sin()),
                    Vec3::new(2., 1.6, 1.7) * scale,
                    0.,
                    if k % 2 == 0 {
                        [46, 172, 19]
                    } else {
                        [98, 216, 21]
                    },
                    Style::Soft,
                );
            }
            // Dark knots and warm vertical highlights lend the trunks a drawn look.
            s.ball(
                r,
                p + Vec3::new(0.15, 2.1, 0.64),
                Vec3::new(0.21, 0.43, 0.04),
                0.,
                [82, 67, 53],
                Style::Ink,
            );
        }
        // Low white fence, gate and a small cottage in the background.
        for i in -10_i32..=10 {
            if i.abs() < 2 {
                continue;
            }
            let x = i as f32 * 1.4;
            self.block(
                r,
                Vec3::new(x, -2.35, -5.),
                Vec3::new(0.10, 1.05, 0.10),
                [218, 211, 173],
            );
        }
        for side in [-1., 1.] {
            for y in [-2.6, -1.6] {
                self.block(
                    r,
                    Vec3::new(side * 8., y, -4.9),
                    Vec3::new(5.9, 0.08, 0.08),
                    [211, 202, 160],
                );
            }
        }
        self.block(
            r,
            Vec3::new(-7.5, -1.1, -10.),
            Vec3::new(2.2, 2.1, 1.5),
            [222, 193, 143],
        );
        s.ball(
            r,
            Vec3::new(-7.5, 1.5, -10.),
            Vec3::new(2.8, 1.2, 2.),
            0.,
            [158, 76, 62],
            Style::Ink,
        );
        self.block(
            r,
            Vec3::new(-7.5, -1.7, -8.45),
            Vec3::new(0.53, 1.45, 0.04),
            [91, 116, 101],
        );
        for x in [-8.9, -6.1] {
            self.block(
                r,
                Vec3::new(x, -0.3, -8.44),
                Vec3::new(0.36, 0.43, 0.06),
                [101, 154, 158],
            );
        }
        // Mushroom clusters on the sides leave the play area readable.
        for k in 0..18 {
            let side = if k % 2 == 0 { -1. } else { 1. };
            let x = side * (7.2 + (k as f32 * 2.7).sin().abs() * 4.);
            let z = 11. - k as f32 * 1.3;
            let size = 0.55 + (k % 3) as f32 * 0.2;
            let p = Vec3::new(x, -3.3, z);
            s.ball(
                r,
                p + Vec3::new(0., size, 0.),
                Vec3::new(0.18, size, 0.18),
                0.,
                [225, 207, 159],
                Style::Ink,
            );
            s.ball(
                r,
                p + Vec3::new(0., size * 2., 0.),
                Vec3::new(size, 0.38, size),
                0.,
                [181, 77, 61],
                Style::Ink,
            );
            for j in [-1., 1.] {
                s.ball(
                    r,
                    p + Vec3::new(j * size * 0.4, size * 2. + 0.26, size * 0.45),
                    Vec3::new(0.13, 0.07, 0.13),
                    0.,
                    [239, 221, 180],
                    Style::Soft,
                );
            }
        }
        // The boss grows from this world rather than floating in outer space.
        let b = g.boss();
        if g.warning {
            use crate::game::Pattern;
            let color = if g.phase_two {
                [242, 130, 91]
            } else {
                [245, 214, 129]
            };
            if g.pattern == Pattern::Rain {
                for lane in 0..6 {
                    if lane == g.rain_gap {
                        continue;
                    }
                    for k in 0..4 {
                        s.ball(
                            r,
                            Vec3::new(-5. + lane as f32 * 2., 5.8 - k as f32 * 0.32, 12.),
                            Vec3::new(0.08, 0.11, 0.08),
                            0.,
                            color,
                            Style::Glow,
                        );
                    }
                }
            } else {
                let count = if g.pattern == Pattern::Ring { 12 } else { 5 };
                for k in 0..count {
                    let a = k as f32 * TAU / count as f32;
                    s.ball(
                        r,
                        b + Vec3::new(a.cos() * 2.75, a.sin() * 2.75, 0.),
                        Vec3::new(0.09, 0.09, 0.09),
                        0.,
                        color,
                        Style::Glow,
                    );
                }
            }
        }
        s.ball(
            r,
            Vec3::new(b.x, -1.4, b.z - 0.45),
            Vec3::new(0.27, 2.2, 0.27),
            0.,
            [67, 116, 73],
            Style::Ink,
        );
        for side in [-1., 1.] {
            s.ball(
                r,
                Vec3::new(b.x + side * 0.8, -1.4, b.z - 0.35),
                Vec3::new(0.95, 0.32, 0.30),
                side * 0.2,
                [89, 136, 79],
                Style::Ink,
            );
        }
        for k in 0..12 {
            let a = k as f32 * TAU / 12. + (g.time * 1.4).sin() * 0.035;
            s.ball(
                r,
                b + Vec3::new(a.cos() * 1.95, a.sin() * 1.95, -0.30),
                Vec3::new(0.70, 0.70, 0.30),
                0.,
                if g.phase_two {
                    [139, 73, 133]
                } else {
                    [186, 86, 60]
                },
                Style::Ink,
            );
        }
        s.ball(
            r,
            b,
            Vec3::new(1.7, 1.7, 0.7),
            0.,
            if g.boss_flash > 0. {
                [255, 247, 218]
            } else {
                [235, 207, 146]
            },
            Style::Ink,
        );
        for side in [-1., 1.] {
            s.ball(
                r,
                b + Vec3::new(side * 0.55, 0.43, 0.65),
                Vec3::new(0.39, 0.51, 0.15),
                0.,
                [249, 236, 195],
                Style::Ink,
            );
            s.ball(
                r,
                b + Vec3::new(side * 0.52, 0.38, 0.81),
                Vec3::new(0.16, 0.27, 0.07),
                0.,
                [39, 34, 32],
                Style::Glow,
            );
            s.ball(
                r,
                b + Vec3::new(side * 0.52 - 0.04, 0.49, 0.89),
                Vec3::new(0.05, 0.07, 0.02),
                0.,
                [255, 244, 211],
                Style::Glow,
            );
            s.ball(
                r,
                b + Vec3::new(side * 0.56, 1.01, 0.48),
                Vec3::new(0.46, 0.10, 0.10),
                0.,
                [63, 46, 34],
                Style::Ink,
            );
            s.ball(
                r,
                b + Vec3::new(side * 1.12, -0.2, 0.52),
                Vec3::new(0.23, 0.16, 0.08),
                0.,
                [208, 125, 86],
                Style::Soft,
            );
        }
        s.ball(
            r,
            b + Vec3::new(0., -0.08, 0.78),
            Vec3::new(0.24, 0.24, 0.25),
            0.,
            [196, 118, 75],
            Style::Ink,
        );
        s.ball(
            r,
            b + Vec3::new(0., -0.69, 0.63),
            Vec3::new(0.83, 0.36, 0.18),
            0.,
            [58, 39, 31],
            Style::Ink,
        );
        for k in -2..=2 {
            s.ball(
                r,
                b + Vec3::new(k as f32 * 0.25, -0.57, 0.80),
                Vec3::new(0.105, 0.13, 0.04),
                0.,
                [249, 232, 184],
                Style::Glow,
            );
        }
        s.ball(
            r,
            Vec3::new(g.player.x, -3.29, g.player.z),
            Vec3::new(1.25, 0.035, 0.8),
            0.,
            [90, 113, 72],
            Style::Glow,
        );
        if g.invulnerable <= 0. || (g.invulnerable * 12.) as i32 % 2 == 0 {
            r.model(&s.ship, g.player, 0.33, 0.);
        }
        for bullet in &g.bullets {
            s.ball(
                r,
                bullet.p,
                if bullet.friendly {
                    Vec3::new(0.12, 0.12, 0.45)
                } else {
                    Vec3::new(0.22, 0.22, 0.22)
                },
                0.,
                if bullet.friendly {
                    [75, 230, 244]
                } else {
                    [197, 59, 78]
                },
                Style::Glow,
            );
        }
        for p in &g.particles {
            s.ball(
                r,
                p.p,
                Vec3::new(0.07, 0.07, 0.07),
                0.,
                p.color,
                Style::Glow,
            );
        }
    }
}
