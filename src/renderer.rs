use crate::{
    math::{Mat4, Vec3},
    obj::Model,
};
use std::f32::consts::PI;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Style {
    Ink,
    Terrain,
    Sun,
    Metal,
    Glow,
    Soft,
    Grass,
    Glass,
    City,
    GardenSurface,
    RunnerSurface,
    MarioSurface,
    Checker,
    Water,
    Character,
}
#[derive(Clone, Copy)]
pub struct Material {
    pub color: [u8; 3],
    pub style: Style,
}
impl Material {
    pub fn new(color: [u8; 3], style: Style) -> Self {
        Self { color, style }
    }
}
pub struct Mesh {
    pub radius: f32,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
}
impl Mesh {
    pub fn cylinder() -> Self {
        let mut m = Self {
            radius: 1.42,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
        };
        for k in 0..=48 {
            let a = k as f32 * std::f32::consts::TAU / 48.;
            let n = Vec3::new(a.cos(), 0., a.sin());
            for y in [-1., 1.] {
                m.vertices.push(n + Vec3::new(0., y, 0.));
                m.normals.push(n);
            }
        }
        for k in 0..48 {
            let j = k * 2;
            m.faces.extend([[j, j + 1, j + 2], [j + 2, j + 1, j + 3]]);
        }
        for y in [-1., 1.] {
            let c = m.vertices.len();
            m.vertices.push(Vec3::new(0., y, 0.));
            m.normals.push(Vec3::new(0., y, 0.));
            for k in 0..=48 {
                let a = k as f32 * std::f32::consts::TAU / 48.;
                m.vertices.push(Vec3::new(a.cos(), y, a.sin()));
                m.normals.push(Vec3::new(0., y, 0.));
            }
            for k in 0..48 {
                m.faces.push(if y > 0. {
                    [c, c + k + 2, c + k + 1]
                } else {
                    [c, c + k + 1, c + k + 2]
                });
            }
        }
        m
    }
    pub fn portal() -> Self {
        let mut m = Self {
            radius: 2.4,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
        };
        for a in 0..48 {
            let angle = a as f32 * 2. * PI / 48.;
            for b in 0..6 {
                let tube = b as f32 * 2. * PI / 6.;
                let n = Vec3::new(
                    angle.cos() * tube.cos(),
                    angle.sin() * tube.cos(),
                    tube.sin(),
                );
                m.vertices
                    .push(Vec3::new(angle.cos() * 2.3, angle.sin() * 2.3, 0.) + n * 0.09);
                m.normals.push(n);
            }
        }
        for a in 0..48 {
            for b in 0..6 {
                let i = a * 6 + b;
                let j = ((a + 1) % 48) * 6 + b;
                let k = a * 6 + (b + 1) % 6;
                let l = ((a + 1) % 48) * 6 + (b + 1) % 6;
                m.faces.extend([[i, j, k], [k, j, l]]);
            }
        }
        m
    }
    pub fn sphere(rings: usize, slices: usize) -> Self {
        let mut m = Self {
            radius: 1.,
            vertices: vec![],
            normals: vec![],
            faces: vec![],
        };
        for r in 0..=rings {
            let lat = PI * r as f32 / rings as f32;
            for s in 0..=slices {
                let lon = 2. * PI * s as f32 / slices as f32;
                let v = Vec3::new(lat.sin() * lon.cos(), lat.cos(), lat.sin() * lon.sin());
                m.vertices.push(v);
                m.normals.push(v);
            }
        }
        for r in 0..rings {
            for s in 0..slices {
                let a = r * (slices + 1) + s;
                let b = a + slices + 1;
                m.faces.push([a, b, a + 1]);
                m.faces.push([a + 1, b, b + 1]);
            }
        }
        m
    }
}
#[derive(Clone, Copy)]
struct Vertex {
    clip: [f32; 4],
    normal: Vec3,
    tangent: Vec3,
    local: Vec3,
    world: Vec3,
}
impl Vertex {
    fn lerp(self, b: Self, t: f32) -> Self {
        Self {
            clip: std::array::from_fn(|i| self.clip[i] + (b.clip[i] - self.clip[i]) * t),
            normal: self.normal.lerp(b.normal, t),
            tangent: self.tangent.lerp(b.tangent, t),
            local: self.local.lerp(b.local, t),
            world: self.world.lerp(b.world, t),
        }
    }
}
#[derive(Default, Clone, Copy)]
pub struct Stats {
    pub width: usize,
    pub height: usize,
    pub objects: usize,
    pub culled: usize,
    pub submitted: usize,
    pub triangles: usize,
    pub fragments: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GpuVertex {
    pub clip: [f32; 4],
    pub local: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub world: [f32; 3],
    pub style: f32,
    pub tangent: [f32; 3],
}
#[derive(Clone)]
pub enum Paint {
    Rect(i32, i32, i32, i32, [u8; 3]),
    Text(i32, i32, String, i32, [u8; 3]),
    Line((i32, i32), (i32, i32), [u8; 3]),
}
pub struct Renderer {
    pub gpu: bool,
    pub gpu_vertices: Vec<GpuVertex>,
    pub background: Vec<Paint>,
    pub overlay: Vec<Paint>,
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
    pub depth: Vec<f32>,
    vp: Mat4,
    pub eye: Vec3,
    parallel_view: Option<Vec3>,
    pub time: f32,
    pub stats: Stats,
    pub culling: bool,
    pub normal_mapping: bool,
}
impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            gpu: false,
            gpu_vertices: Vec::new(),
            background: Vec::new(),
            overlay: Vec::new(),
            width,
            height,
            pixels: vec![0; width * height * 4],
            depth: vec![f32::INFINITY; width * height],
            vp: Mat4::model(Vec3::default(), Vec3::new(1., 1., 1.), 0.),
            eye: Vec3::default(),
            parallel_view: None,
            time: 0.,
            stats: Stats::default(),
            culling: true,
            normal_mapping: true,
        }
    }
    pub fn begin(&mut self, eye: Vec3, target: Vec3, time: f32) {
        self.gpu_vertices.clear();
        self.background.clear();
        self.overlay.clear();
        self.eye = eye;
        self.parallel_view = None;
        self.time = time;
        self.stats = Stats {
            width: self.width,
            height: self.height,
            ..Stats::default()
        };
        if !self.gpu {
            self.depth.fill(f32::INFINITY);
        }
        self.vp = Mat4::perspective(
            self.width as f32 / self.height as f32,
            60_f32.to_radians(),
            0.15,
            300.,
        ) * Mat4::view(eye, target);
        // Royal-blue to turquoise illustrated sky; shared by the CPU and GPU paths.
        for y in 0..self.height {
            let t = y as f32 / self.height as f32;
            self.rect(
                0,
                y as i32,
                self.width as i32,
                1,
                [
                    (18. + 25. * t) as u8,
                    (57. + 146. * t) as u8,
                    (178. + 62. * t) as u8,
                ],
            );
        }
        // Fixed world-space stars, so rotating the camera changes the sky naturally.
        for i in 0..300 {
            let a = i as f32 * 2.399963;
            let y = 1. - 2. * (i as f32 + 0.5) / 300.;
            let s = (1. - y * y).sqrt();
            let p = self.eye + Vec3::new(s * a.cos(), y, s * a.sin()) * 160.;
            if let Some((x, y, _)) = self.project(p) {
                let c = if i % 6 == 0 {
                    [255, 238, 153]
                } else {
                    [155, 219, 255]
                };
                self.pixel(x as i32, y as i32, c);
                if i % 19 == 0 {
                    self.rect(x as i32 - 2, y as i32, 5, 1, c);
                    self.rect(x as i32, y as i32 - 2, 1, 5, c);
                }
            }
        }
    }
    pub fn begin_ortho(&mut self, eye: Vec3, target: Vec3, time: f32, height: f32) {
        self.begin(eye, target, time);
        self.parallel_view = Some(eye - target);
        self.vp = Mat4::orthographic(self.width as f32 / self.height as f32, height, 0.15, 300.)
            * Mat4::view(eye, target);
    }
    pub fn project(&self, p: Vec3) -> Option<(f32, f32, f32)> {
        let c = self.vp.transform(p);
        if c[3] <= 0.15 {
            return None;
        }
        let n = [c[0] / c[3], c[1] / c[3], c[2] / c[3]];
        if n.iter().any(|v| v.abs() > 1.) {
            return None;
        }
        Some((
            (n[0] * 0.5 + 0.5) * self.width as f32,
            (0.5 - n[1] * 0.5) * self.height as f32,
            n[2],
        ))
    }
    fn visible(&self, pos: Vec3, radius: f32) -> bool {
        if !self.culling {
            return true;
        }
        // Six frustum planes extracted from VP. Sphere rejection precedes vertex processing.
        for (axis, sign) in [(0, 1.), (0, -1.), (1, 1.), (1, -1.), (2, 1.), (2, -1.)] {
            let p: [f32; 4] = std::array::from_fn(|i| self.vp.0[3][i] + sign * self.vp.0[axis][i]);
            let norm = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            if p[0] * pos.x + p[1] * pos.y + p[2] * pos.z + p[3] < -radius * norm {
                return false;
            }
        }
        true
    }
    pub fn mesh(&mut self, m: &Mesh, pos: Vec3, scale: Vec3, yaw: f32, mat: Material) {
        self.stats.objects += 1;
        let radius = m.radius * scale.x.abs().max(scale.y.abs()).max(scale.z.abs());
        if !self.visible(pos, radius) {
            self.stats.culled += 1;
            return;
        }
        let model = Mat4::model(pos, scale, yaw);
        let mvp = self.vp * model;
        let vertices: Vec<_> = m
            .vertices
            .iter()
            .zip(&m.normals)
            .map(|(&p, &n)| {
                let w = model.transform(p);
                Vertex {
                    clip: mvp.transform(p),
                    world: Vec3::new(w[0], w[1], w[2]),
                    local: p,
                    tangent: {
                        let t = crate::normal_map::tangent(p);
                        Vec3::new(t.x * scale.x, t.y * scale.y, t.z * scale.z)
                            .rotate_y(yaw)
                            .normalized()
                    },
                    normal: Vec3::new(n.x / scale.x, n.y / scale.y, n.z / scale.z)
                        .rotate_y(yaw)
                        .normalized(),
                }
            })
            .collect();
        for face in &m.faces {
            self.stats.submitted += 1;
            let vs = face.map(|i| vertices[i]);
            let mut outward = (vs[1].world - vs[0].world).cross(vs[2].world - vs[0].world);
            if outward.dot(vs[0].normal + vs[1].normal + vs[2].normal) < 0. {
                outward = outward * (-1.);
            }
            if outward.dot(self.parallel_view.unwrap_or(self.eye - vs[0].world)) <= 0. {
                continue;
            }
            self.clip_triangle(vs, mat);
        }
    }
    pub fn model(&mut self, m: &Model, pos: Vec3, scale: f32, yaw: f32) {
        self.oriented_model(m, pos, scale, yaw, 0., 0.);
    }
    pub fn oriented_model(
        &mut self,
        m: &Model,
        pos: Vec3,
        scale: f32,
        yaw: f32,
        pitch: f32,
        bank: f32,
    ) {
        self.stats.objects += 1;
        let radius = m.radius * scale.abs();
        if !self.visible(pos, radius) {
            self.stats.culled += 1;
            return;
        }
        let model = Mat4::model(pos, Vec3::new(scale, scale, scale), yaw)
            * Mat4::rotation_x(pitch)
            * Mat4::rotation_z(bank);
        let mvp = self.vp * model;
        let positions: Vec<_> = m
            .vertices
            .iter()
            .map(|&p| {
                let w = model.transform(p);
                (mvp.transform(p), Vec3::new(w[0], w[1], w[2]))
            })
            .collect();
        let normals: Vec<_> = m
            .normals
            .iter()
            .map(|n| n.rotate_z(bank).rotate_x(pitch).rotate_y(yaw))
            .collect();
        for f in &m.faces {
            self.stats.submitted += 1;
            let ps = f.vertices.map(|i| positions[i].1);
            let geometric = (ps[1] - ps[0]).cross(ps[2] - ps[0]);
            if geometric.length() < 1e-8 {
                continue;
            }
            let fallback = geometric.normalized();
            let corner_normals =
                std::array::from_fn::<_, 3, _>(|i| f.normals[i].map_or(fallback, |j| normals[j]));
            let outward =
                if geometric.dot(corner_normals[0] + corner_normals[1] + corner_normals[2]) < 0. {
                    geometric * -1.
                } else {
                    geometric
                };
            if outward.dot(self.parallel_view.unwrap_or(self.eye - ps[0])) <= 0. {
                continue;
            }
            let vs = std::array::from_fn(|i| {
                let index = f.vertices[i];
                Vertex {
                    clip: positions[index].0,
                    world: ps[i],
                    normal: corner_normals[i],
                    tangent: crate::normal_map::tangent(m.vertices[index])
                        .rotate_z(bank)
                        .rotate_x(pitch)
                        .rotate_y(yaw),
                    local: m.vertices[index],
                }
            });
            let color = m
                .colors
                .get(&f.material)
                .copied()
                .unwrap_or([222, 188, 130]);
            let style = match f.material.as_str() {
                "engine" | "redlight" => Style::Glow,
                "glass" => Style::Glass,
                name if name.starts_with("glow_") => Style::Glow,
                "checker_cliff" => Style::Checker,
                "water_blue" => Style::Water,
                "surface_garden" => Style::GardenSurface,
                "surface_runner" => Style::RunnerSurface,
                "surface_mario" => Style::MarioSurface,
                name if name.starts_with("city_") => Style::City,
                name if name.starts_with("terrain_") => Style::Terrain,
                name if name.starts_with("hero_") => Style::Character,
                name if name.starts_with("paint_") => Style::Soft,
                _ => Style::Metal,
            };
            self.clip_triangle(vs, Material::new(color, style));
        }
    }
    fn clip_triangle(&mut self, vs: [Vertex; 3], mat: Material) {
        if self.gpu {
            self.stats.triangles += 1;
            for v in vs {
                self.gpu_vertices.push(GpuVertex {
                    clip: v.clip,
                    local: [v.local.x, v.local.y, v.local.z],
                    normal: [v.normal.x, v.normal.y, v.normal.z],
                    color: [
                        mat.color[0] as f32 / 255.,
                        mat.color[1] as f32 / 255.,
                        mat.color[2] as f32 / 255.,
                        1.,
                    ],
                    world: [v.world.x, v.world.y, v.world.z],
                    style: mat.style as u32 as f32,
                    tangent: [v.tangent.x, v.tangent.y, v.tangent.z],
                });
            }
            return;
        }
        let mut entirely_inside = true;
        for (axis, sign) in [(0, 1.), (0, -1.), (1, 1.), (1, -1.), (2, 1.), (2, -1.)] {
            let inside = vs
                .iter()
                .filter(|v| v.clip[3] + sign * v.clip[axis] >= 0.)
                .count();
            if inside == 0 {
                return;
            }
            entirely_inside &= inside == 3;
        }
        if entirely_inside {
            self.triangle(vs, mat);
            return;
        }
        let mut poly = vs.to_vec();
        for (axis, sign) in [(0, 1.), (0, -1.), (1, 1.), (1, -1.), (2, 1.), (2, -1.)] {
            if poly.is_empty() {
                return;
            }
            let input = std::mem::take(&mut poly);
            let mut prev = *input.last().unwrap();
            let mut pd = prev.clip[3] + sign * prev.clip[axis];
            for curr in input {
                let cd = curr.clip[3] + sign * curr.clip[axis];
                if (cd >= 0.) != (pd >= 0.) {
                    poly.push(prev.lerp(curr, pd / (pd - cd)));
                }
                if cd >= 0. {
                    poly.push(curr)
                }
                prev = curr;
                pd = cd;
            }
        }
        for i in 1..poly.len().saturating_sub(1) {
            self.triangle([poly[0], poly[i], poly[i + 1]], mat);
        }
    }
    fn triangle(&mut self, v: [Vertex; 3], mat: Material) {
        let screen = v.map(|v| {
            Vec3::new(
                (v.clip[0] / v.clip[3] * 0.5 + 0.5) * self.width as f32,
                (0.5 - v.clip[1] / v.clip[3] * 0.5) * self.height as f32,
                v.clip[2] / v.clip[3],
            )
        });
        let edge =
            |a: Vec3, b: Vec3, x: f32, y: f32| (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x);
        let area = edge(screen[0], screen[1], screen[2].x, screen[2].y);
        if !area.is_finite() || area.abs() < 0.01 {
            return;
        }
        self.stats.triangles += 1;
        let xmin = screen
            .iter()
            .map(|v| v.x)
            .fold(f32::INFINITY, f32::min)
            .floor()
            .max(0.) as usize;
        let xmax = screen
            .iter()
            .map(|v| v.x)
            .fold(f32::NEG_INFINITY, f32::max)
            .ceil()
            .min(self.width as f32 - 1.) as usize;
        let ymin = screen
            .iter()
            .map(|v| v.y)
            .fold(f32::INFINITY, f32::min)
            .floor()
            .max(0.) as usize;
        let ymax = screen
            .iter()
            .map(|v| v.y)
            .fold(f32::NEG_INFINITY, f32::max)
            .ceil()
            .min(self.height as f32 - 1.) as usize;
        // Smooth scenery uses vertex lighting; the cartoon, sun and terrain
        // still execute their distinct fragment shaders per visible sample.
        let vertex_colors = if matches!(mat.style, Style::Soft | Style::Grass | Style::Metal) {
            Some(v.map(|vertex| {
                fragment(
                    mat,
                    vertex.normal,
                    vertex.local,
                    vertex.world,
                    self.eye,
                    self.time,
                    0,
                    0,
                )
            }))
        } else {
            None
        };
        let reciprocal_w = v.map(|vertex| 1. / vertex.clip[3]);
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let a = edge(screen[1], screen[2], x as f32 + 0.5, y as f32 + 0.5) / area;
                let b = edge(screen[2], screen[0], x as f32 + 0.5, y as f32 + 0.5) / area;
                let c = 1. - a - b;
                if a < 0. || b < 0. || c < 0. {
                    continue;
                }
                let z = a * screen[0].z + b * screen[1].z + c * screen[2].z;
                let idx = y * self.width + x;
                if z >= self.depth[idx] {
                    continue;
                }
                let color = if matches!(mat.style, Style::Glow) {
                    mat.color
                } else {
                    let weights = [
                        a * reciprocal_w[0],
                        b * reciprocal_w[1],
                        c * reciprocal_w[2],
                    ];
                    let inverse_sum = 1. / weights.iter().sum::<f32>();
                    let w = weights.map(|value| value * inverse_sum);
                    if let Some(colors) = vertex_colors {
                        std::array::from_fn(|channel| {
                            (colors[0][channel] as f32 * w[0]
                                + colors[1][channel] as f32 * w[1]
                                + colors[2][channel] as f32 * w[2])
                                .clamp(0., 255.) as u8
                        })
                    } else {
                        let mut n = (v[0].normal * w[0] + v[1].normal * w[1] + v[2].normal * w[2])
                            .normalized();
                        let p = v[0].local * w[0] + v[1].local * w[1] + v[2].local * w[2];
                        let world = v[0].world * w[0] + v[1].world * w[1] + v[2].world * w[2];
                        if self.normal_mapping
                            && matches!(
                                mat.style,
                                Style::GardenSurface | Style::RunnerSurface | Style::MarioSurface
                            )
                        {
                            let t = v[0].tangent * w[0] + v[1].tangent * w[1] + v[2].tangent * w[2];
                            n = crate::normal_map::mapped(n, t, p);
                        }
                        fragment(mat, n, p, world, self.eye, self.time, x, y)
                    }
                };
                self.depth[idx] = z;
                self.pixels[idx * 4..idx * 4 + 4]
                    .copy_from_slice(&[color[0], color[1], color[2], 255]);
                self.stats.fragments += 1;
            }
        }
    }
    pub fn paint(&mut self, p: Paint) {
        if self.gpu_vertices.is_empty() {
            self.background.push(p);
        } else {
            self.overlay.push(p);
        }
    }
    pub fn pixel(&mut self, x: i32, y: i32, c: [u8; 3]) {
        if self.gpu {
            self.paint(Paint::Rect(x, y, 1, 1, c));
            return;
        }
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let i = (y as usize * self.width + x as usize) * 4;
            self.pixels[i..i + 4].copy_from_slice(&[c[0], c[1], c[2], 255]);
        }
    }
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: [u8; 3]) {
        if self.gpu {
            self.paint(Paint::Rect(x, y, w, h, c));
            return;
        }
        for yy in y.max(0)..(y + h).min(self.height as i32) {
            for xx in x.max(0)..(x + w).min(self.width as i32) {
                self.pixel(xx, yy, c)
            }
        }
    }
    pub fn line(&mut self, a: (i32, i32), b: (i32, i32), c: [u8; 3]) {
        if self.gpu {
            self.paint(Paint::Line(a, b, c));
            return;
        }
        let steps = (b.0 - a.0).abs().max((b.1 - a.1).abs()).max(1);
        for i in 0..=steps {
            self.pixel(
                a.0 + (b.0 - a.0) * i / steps,
                a.1 + (b.1 - a.1) * i / steps,
                c,
            );
        }
    }
    /// Small, separable CPU bloom for engine and projectile emission only.
    pub fn engine_glow(&mut self) {
        if self.gpu {
            return;
        }
        let w = self.width / 4;
        let h = self.height / 4;
        let mut a = vec![0.0f32; w * h];
        let mut b = vec![0.0f32; w * h];
        for y in 0..h {
            for x in 0..w {
                let mut energy: f32 = 0.;
                for yy in 0..4 {
                    for xx in 0..4 {
                        let i = ((y * 4 + yy) * self.width + x * 4 + xx) * 4;
                        let p = &self.pixels[i..i + 3];
                        if p[2] > 205 && p[1] > 170 && (p[2] as f32) > p[0] as f32 * 1.3 {
                            energy += 1.;
                        }
                    }
                }
                a[y * w + x] = energy / 16.;
            }
        }
        for y in 0..h {
            for x in 0..w {
                for k in -4_i32..=4 {
                    let xx = (x as i32 + k).clamp(0, w as i32 - 1) as usize;
                    b[y * w + x] += a[y * w + xx] * (5 - k.abs()) as f32 / 25.;
                }
            }
        }
        a.fill(0.);
        for y in 0..h {
            for x in 0..w {
                for k in -4_i32..=4 {
                    let yy = (y as i32 + k).clamp(0, h as i32 - 1) as usize;
                    a[y * w + x] += b[yy * w + x] * (5 - k.abs()) as f32 / 25.;
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let light = a[(y / 4).min(h - 1) * w + (x / 4).min(w - 1)];
                let i = (y * self.width + x) * 4;
                self.pixels[i] = self.pixels[i].saturating_add((light * 12.) as u8);
                self.pixels[i + 1] = self.pixels[i + 1].saturating_add((light * 95.) as u8);
                self.pixels[i + 2] = self.pixels[i + 2].saturating_add((light * 145.) as u8);
            }
        }
    }
    pub fn save(&self, path: &std::path::Path) -> Result<(), image::ImageError> {
        image::save_buffer(
            path,
            &self.pixels,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgba8,
        )
    }
}
// Fragment shaders execute once per visible sample, on the CPU.
#[allow(clippy::too_many_arguments)]
fn fragment(
    mat: Material,
    n: Vec3,
    p: Vec3,
    world: Vec3,
    eye: Vec3,
    time: f32,
    x: usize,
    y: usize,
) -> [u8; 3] {
    let facing = if matches!(mat.style, Style::Ink | Style::Terrain | Style::Glass) {
        n.dot((eye - world).normalized()).abs()
    } else {
        0.
    };
    let light = n.dot(Vec3::new(-0.5, 0.8, 0.6).normalized());
    let mut base = mat.color;
    let intensity;
    match mat.style {
        Style::Sun => {
            let bands = (p.y * 24. + time * 1.6 + (p.x * 9.).sin() * 2.).sin() * 0.5 + 0.5;
            return [255, (157. + bands * 65.) as u8, (56. + bands * 58.) as u8];
        }
        Style::Glow => return base,
        Style::Terrain => {
            let noise = (p.x * 9. + (p.z * 7.).sin()).sin() + (p.y * 11. + p.z * 3.).cos() * 0.65;
            base = if noise > 0.3 {
                [104, 151, 128]
            } else if noise > -0.4 {
                [212, 179, 120]
            } else {
                [179, 119, 88]
            };
            if facing < 0.14 {
                return [39, 39, 44];
            };
            intensity = if light > 0.45 {
                1.0
            } else if light > -0.1 {
                0.82
            } else {
                0.59
            };
        }
        Style::Ink => {
            if facing < 0.20 {
                return [39, 34, 38];
            };
            intensity = if light > 0.4 {
                1.
            } else if light > -0.2 {
                0.8
            } else {
                0.57
            };
        }
        Style::Metal => {
            let half =
                ((eye - world).normalized() + Vec3::new(-0.5, 0.8, 0.6).normalized()).normalized();
            let spec = n.dot(half).max(0.).powi(24) * 0.32;
            intensity = 0.48 + 0.52 * light.max(0.) + spec;
        }
        Style::City => {
            let horizontal = if n.x.abs() > n.z.abs() { p.z } else { p.x };
            let column = (horizontal * 2.5).rem_euclid(1.);
            let row = (p.y * 3.).rem_euclid(1.);
            let window = column > 0.22 && column < 0.50 && row > 0.28 && row < 0.57;
            let pulse = 0.85 + 0.15 * (time * 0.7 + (horizontal * 2.5).floor()).sin();
            if window && n.y.abs() < 0.85 {
                return [75, (192. * pulse) as u8, (211. * pulse) as u8];
            }
            base = if p.y > 0.45 {
                [105, 75, 128]
            } else {
                [44, 43, 69]
            };
            intensity = 0.65 + 0.35 * light.max(0.);
        }

        Style::Glass => {
            let reflection = n.y.abs() * 0.5 + facing * 0.35;
            return [
                (35. + reflection * 75.) as u8,
                (92. + reflection * 110.) as u8,
                (132. + reflection * 105.) as u8,
            ];
        }
        Style::GardenSurface | Style::RunnerSurface | Style::MarioSurface => {
            let unit = p.normalized();
            let q = (unit.x * 5. + unit.z * 2.).sin() * (unit.y * 4. - unit.z * 3.).cos();
            let coast = -0.18 + 0.16 * (unit.x * 6. + unit.z * 4.).sin();
            let check = ((unit.z.atan2(unit.x) * 5.092958).floor()
                + (unit.y.clamp(-1., 1.).asin() * 7.639437).floor())
            .rem_euclid(2.);
            base = if unit.y < coast {
                if check < 1. {
                    [126, 52, 16]
                } else {
                    [205, 111, 37]
                }
            } else if unit.y < coast + 0.055 {
                [153, 246, 29]
            } else if q < -0.45 {
                [10, 201, 224]
            } else if q < -0.38 {
                [255, 222, 108]
            } else if q > 0.25 {
                [112, 228, 18]
            } else {
                [55, 193, 22]
            };
            intensity = 0.70 + 0.30 * light.max(0.);
        }
        Style::Checker => {
            let check =
                ((p.x * 2.).floor() + (p.y * 2.).floor() + (p.z * 2.).floor()).rem_euclid(2.);
            base = if check < 1. {
                [126, 52, 16]
            } else {
                [205, 111, 37]
            };
            intensity = if light > 0.4 { 1. } else { 0.8 };
        }
        Style::Water => {
            let wave = (world.y * 13. + world.x * 5. - time * 4.).sin();
            base = if wave > 0.8 {
                [143, 250, 255]
            } else {
                [10, 199, 225]
            };
            intensity = 0.95;
        }
        Style::Character => {
            intensity = 0.70 + 0.30 * light.max(0.);
        }
        Style::Soft => {
            intensity = if light > 0.5 {
                1.
            } else if light > -0.1 {
                0.88
            } else {
                0.72
            };
        }
        Style::Grass => {
            let stripe = (world.x * 0.30 + (world.z * 0.18).sin() * 1.4).sin();
            let speck = (world.x * 14.).sin() * (world.z * 17.).sin();
            base = if stripe > 0. {
                [91, 223, 17]
            } else {
                [49, 193, 22]
            };
            intensity = 0.88 + 0.12 * light.max(0.) + speck * 0.025;
        }
    }
    let _ = (x, y);
    let grain = 0.;
    base.map(|c| (c as f32 * intensity + grain).clamp(0., 255.) as u8)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn depth_is_order_independent() {
        let m = Mesh::sphere(8, 12);
        let mut r = Renderer::new(80, 60);
        let draw = |r: &mut Renderer, rev: bool| {
            r.begin(Vec3::new(0., 0., 8.), Vec3::default(), 0.);
            for z in if rev { [0., 2.] } else { [2., 0.] } {
                r.mesh(
                    &m,
                    Vec3::new(0., 0., z),
                    Vec3::new(1., 1., 1.),
                    0.,
                    Material::new(if z == 0. { [255, 0, 0] } else { [0, 255, 0] }, Style::Glow),
                );
            }
        };
        draw(&mut r, false);
        let pixels = r.pixels.clone();
        draw(&mut r, true);
        assert_eq!(pixels, r.pixels);
        assert_eq!(
            &r.pixels[(30 * 80 + 40) * 4..(30 * 80 + 40) * 4 + 3],
            &[0, 255, 0]
        );
    }
    #[test]
    fn normal_texture_changes_lighting_without_changing_geometry() {
        let mesh = Mesh::sphere(20, 32);
        let render = |enabled| {
            let mut r = Renderer::new(160, 120);
            r.normal_mapping = enabled;
            r.begin(Vec3::new(0., 0., 3.), Vec3::default(), 0.);
            r.mesh(
                &mesh,
                Vec3::default(),
                Vec3::new(1., 1., 1.),
                0.7,
                Material::new([55, 193, 22], Style::GardenSurface),
            );
            r
        };
        let flat = render(false);
        let mapped = render(true);
        assert_eq!(
            flat.depth, mapped.depth,
            "normal maps must not change silhouette or depth"
        );
        let changed = flat
            .pixels
            .chunks_exact(4)
            .zip(mapped.pixels.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            changed > 500,
            "expected visible per-fragment lighting variation, got {changed}"
        );
        assert_eq!(std::mem::size_of::<GpuVertex>(), 84);
    }
    #[test]
    fn near_plane_clipping_keeps_crossing_triangle() {
        let mut r = Renderer::new(80, 60);
        r.begin(Vec3::default(), Vec3::new(0., 0., -1.), 0.);
        let mk = |p| Vertex {
            clip: r.vp.transform(p),
            normal: Vec3::new(0., 0., 1.),
            tangent: Vec3::new(1., 0., 0.),
            local: p,
            world: p,
        };
        let v = [
            mk(Vec3::new(-0.5, -0.5, -1.)),
            mk(Vec3::new(0.5, -0.5, -1.)),
            mk(Vec3::new(0., 0.3, 0.1)),
        ];
        r.clip_triangle(v, Material::new([255, 0, 0], Style::Glow));
        assert!(r.stats.fragments > 0);
        assert!(
            r.depth
                .iter()
                .filter(|z| z.is_finite())
                .all(|z| *z >= -1.001 && *z <= 1.001)
        );
    }
    #[test]
    fn culling_does_not_change_image() {
        let m = Mesh::sphere(8, 12);
        let mut r = Renderer::new(80, 60);
        r.begin(Vec3::new(0., 0., 8.), Vec3::default(), 0.);
        r.mesh(
            &m,
            Vec3::new(100., 0., 0.),
            Vec3::new(1., 1., 1.),
            0.,
            Material::new([255, 0, 0], Style::Ink),
        );
        let pixels = r.pixels.clone();
        assert_eq!(r.stats.culled, 1);
        r.culling = false;
        r.mesh(
            &m,
            Vec3::new(100., 0., 0.),
            Vec3::new(1., 1., 1.),
            0.,
            Material::new([255, 0, 0], Style::Ink),
        );
        assert_eq!(pixels, r.pixels);
    }
}
