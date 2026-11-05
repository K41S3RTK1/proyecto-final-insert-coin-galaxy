use crate::{
    game::{Game, WorldId},
    math::Vec3,
    obj::Model,
    renderer::{Material, Mesh, Renderer, Style},
};
use std::f32::consts::{PI, TAU};
pub struct Scene {
    pub(crate) world: crate::world::World,
    planets: Vec<(Model, Model)>,
    portal: Mesh,
    pub(crate) cylinder: Mesh,
    tiny: Mesh,
    sphere: Mesh,
    medium: Mesh,
    detail: Mesh,
    pub race: crate::race_scene::Visuals,
    pub ship: Model,
    palm: Model,
    hill_loop: Model,
    pub lod: bool,
}
impl Scene {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let bundled = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("../Resources/assets");
        let assets = if bundled.exists() {
            bundled
        } else {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
        };
        let mut planets = Vec::new();
        for name in ["garden", "dojo", "runner", "mario", "race"] {
            planets.push((
                Model::load(&assets.join(format!("models/{name}.obj")))?,
                Model::load(&assets.join(format!("models/{name}-low.obj")))?,
            ));
        }
        Ok(Self {
            world: crate::world::World::new(),
            race: crate::race_scene::Visuals::new(&assets)?,
            planets,
            portal: Mesh::portal(),
            cylinder: Mesh::cylinder(),
            tiny: Mesh::sphere(4, 6),
            sphere: Mesh::sphere(8, 12),
            medium: Mesh::sphere(14, 20),
            detail: Mesh::sphere(32, 48),
            palm: Model::load(&assets.join("models/hill-palm.obj"))?,
            hill_loop: Model::load(&assets.join("models/hill-loop.obj"))?,
            ship: Model::load(&assets.join("arcade-ship.obj"))?,
            lod: true,
        })
    }
    pub(crate) fn ball(
        &self,
        r: &mut Renderer,
        pos: Vec3,
        scale: Vec3,
        yaw: f32,
        color: [u8; 3],
        style: Style,
    ) {
        // Approximate projected radius: small decorations need very few vertices.
        let pixels = scale.x.max(scale.y).max(scale.z) * r.height as f32 * 0.866
            / (pos - r.eye).length().max(0.1);
        let mesh = if !self.lod || pixels > 85. {
            &self.detail
        } else if pixels > 24. {
            &self.medium
        } else if pixels < 3. {
            &self.tiny
        } else {
            &self.sphere
        };
        r.mesh(mesh, pos, scale, yaw, Material::new(color, style));
    }
    pub(crate) fn palm(&self, r: &mut Renderer, p: Vec3, scale: f32) {
        r.model(&self.palm, p, scale, 0.);
    }
    pub(crate) fn hill_loop(&self, r: &mut Renderer, p: Vec3, scale: f32) {
        r.model(&self.hill_loop, p, scale, 0.);
    }
    fn sky_clouds(&self, r: &mut Renderer) {
        for k in 0..14 {
            let a = k as f32 * 2.399963;
            let y = -0.15 + (k % 4) as f32 * 0.16;
            let center = r.eye + Vec3::new(a.cos(), y, a.sin()).normalized() * 135.;
            for j in -2_i32..=2 {
                self.ball(
                    r,
                    center + Vec3::new(j as f32 * 3.6, if j == 0 { 1.7 } else { 0. }, 0.),
                    Vec3::new(5., 2.5, 3.),
                    0.,
                    [228, 250, 255],
                    Style::Soft,
                );
            }
        }
    }
    pub fn preview_ship(&self, r: &mut Renderer) {
        r.begin(Vec3::new(4.7, 4., 6.2), Vec3::new(0., 0., -0.4), 0.);
        r.model(&self.ship, Vec3::default(), 1., 0.);
        r.engine_glow();
    }
    pub fn preview_asset(&self, r: &mut Renderer, name: &str) {
        if ["sonic-model", "tails-model", "knuckles-model"].contains(&name) {
            crate::race_scene::preview(self, r, name);
            return;
        }
        if name == "ship" {
            self.preview_ship(r);
            return;
        }
        let index = match name {
            "garden-model" => 0,
            "dojo-model" => 1,
            "runner-model" => 2,
            _ => 3,
        };
        r.begin(Vec3::new(10., 8., 17.), Vec3::new(0., 0.8, 0.), 0.);
        r.model(&self.planets[index].0, Vec3::default(), 1., -0.20);
    }
    pub fn render(&self, r: &mut Renderer, g: &Game) {
        if g.inside() {
            if g.active_world == WorldId::Race {
                crate::race_scene::render(self, r, g);
            } else if g.active_world == WorldId::Garden {
                self.world.render(self, r, g);
            } else if g.active_world == WorldId::Dojo {
                crate::dojo_scene::render(self, r, g);
            } else if g.active_world == WorldId::Mario {
                crate::platform_scene::render(self, r, g);
            } else {
                crate::runner_scene::render(self, r, g);
            }
            return;
        }
        r.begin(g.camera_eye, g.camera_target, g.time);
        self.sky_clouds(r);
        self.ball(
            r,
            Vec3::default(),
            Vec3::new(4.6, 4.6, 4.6),
            g.time * 0.1,
            [255, 198, 87],
            Style::Sun,
        );
        // Coin-like corona made from real geometry, shaded and depth-tested on CPU.
        for i in 0..32 {
            let a = i as f32 * TAU / 32. + g.time * 0.15;
            self.ball(
                r,
                Vec3::new(a.cos() * 5.8, a.sin() * 5.8, 0.),
                Vec3::new(0.09, 0.09, 0.09),
                0.,
                [244, 196, 113],
                Style::Glow,
            );
        }
        for id in WorldId::ALL {
            let pair = &self.planets[id.index()];
            let model = if self.lod && (g.planet_at(id) - r.eye).length() > 42. {
                &pair.1
            } else {
                &pair.0
            };
            r.model(model, g.planet_at(id), 1., g.spin_at(id));
        }
        for id in WorldId::ALL {
            r.mesh(
                &self.portal,
                g.world_at(id, Vec3::new(0., 1., 10.)),
                Vec3::new(1., 1., 1.),
                g.spin_at(id),
                Material::new([255, 196, 29], Style::Metal),
            );
            for k in 0..12 {
                let a = k as f32 * TAU / 12. + g.time * 0.3;
                let p = Vec3::new(a.cos() * 2.3, 1. + a.sin() * 2.3, 10.);
                self.ball(
                    r,
                    g.world_at(id, p),
                    Vec3::new(0.07, 0.07, 0.07),
                    0.,
                    id.color(),
                    Style::Glow,
                );
            }
        }
        r.oriented_model(&self.ship, g.ship, 0.55, -g.yaw, g.pitch, g.bank());
        let thrust = (g.velocity.length() / 16.).clamp(0., 1.);
        if thrust > 0.03 {
            for k in 0..5 {
                let t = k as f32 / 5.;
                let p = g.ship
                    + Vec3::new(0., -0.06, 1.5 + t * (0.6 + thrust * 2.))
                        .rotate_x(g.pitch)
                        .rotate_y(-g.yaw);
                self.ball(
                    r,
                    p,
                    Vec3::new(0.72 * (1. - t * 0.6), 0.045, 0.24 + thrust * 0.15),
                    -g.yaw,
                    [73, (211. - t * 80.) as u8, (255. - t * 80.) as u8],
                    Style::Glow,
                );
            }
        }
        for b in &g.bullets {
            self.ball(
                r,
                g.world(b.p),
                if b.friendly {
                    Vec3::new(0.10, 0.10, 0.32)
                } else {
                    Vec3::new(0.20, 0.20, 0.20)
                },
                g.spin(),
                if b.friendly {
                    [112, 245, 215]
                } else {
                    [249, 128, 100]
                },
                Style::Glow,
            );
        }
        // In-world approach marker, hidden during the challenge.
        if let Some((x, y, _)) = r.project(g.gate()) {
            let c = [126, 210, 188];
            r.line((x as i32 - 5, y as i32), (x as i32, y as i32 - 5), c);
            r.line((x as i32, y as i32 - 5), (x as i32 + 5, y as i32), c);
            r.line((x as i32 + 5, y as i32), (x as i32, y as i32 + 5), c);
            r.line((x as i32, y as i32 + 5), (x as i32 - 5, y as i32), c);
        }
        // A subtle mechanical orbit guide, projected in world space.
        for k in 0..120 {
            let a = k as f32 * PI / 60.;
            if let Some((x, y, _)) = r.project(Vec3::new(a.cos() * 22., -0.1, a.sin() * 22.)) {
                let idx = y as usize * r.width + x as usize;
                if idx < r.depth.len() && r.depth[idx] == f32::INFINITY {
                    r.pixel(x as i32, y as i32, [136, 213, 255]);
                }
            }
        }
    }
}
