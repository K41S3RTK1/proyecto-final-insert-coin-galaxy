use std::ops::{Add, Mul, Sub};
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn rotate_x(self, a: f32) -> Self {
        Self::new(
            self.x,
            self.y * a.cos() - self.z * a.sin(),
            self.y * a.sin() + self.z * a.cos(),
        )
    }
    pub fn rotate_z(self, a: f32) -> Self {
        Self::new(
            self.x * a.cos() - self.y * a.sin(),
            self.x * a.sin() + self.y * a.cos(),
            self.z,
        )
    }
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn lerp(self, b: Self, t: f32) -> Self {
        self + (b - self) * t
    }
    pub fn rotate_y(self, a: f32) -> Self {
        Self::new(
            self.x * a.cos() + self.z * a.sin(),
            self.y,
            -self.x * a.sin() + self.z * a.cos(),
        )
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    pub fn cross(self, b: Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
    pub fn normalized(self) -> Self {
        let l = self.dot(self).sqrt();
        if l > 0.0 { self * (1.0 / l) } else { self }
    }
}

/// Column-vector matrix; model, view and projection are multiplied explicitly.
#[derive(Clone, Copy)]
pub struct Mat4(pub [[f32; 4]; 4]);
impl Mat4 {
    pub fn rotation_x(a: f32) -> Self {
        let (s, c) = a.sin_cos();
        Self([
            [1., 0., 0., 0.],
            [0., c, -s, 0.],
            [0., s, c, 0.],
            [0., 0., 0., 1.],
        ])
    }
    pub fn rotation_z(a: f32) -> Self {
        let (s, c) = a.sin_cos();
        Self([
            [c, -s, 0., 0.],
            [s, c, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }
    pub fn transform(self, p: Vec3) -> [f32; 4] {
        let v = [p.x, p.y, p.z, 1.0];
        std::array::from_fn(|r| (0..4).map(|c| self.0[r][c] * v[c]).sum())
    }
    pub fn model(pos: Vec3, scale: Vec3, yaw: f32) -> Self {
        let (s, c) = yaw.sin_cos();
        Self([
            [c * scale.x, 0., s * scale.z, pos.x],
            [0., scale.y, 0., pos.y],
            [-s * scale.x, 0., c * scale.z, pos.z],
            [0., 0., 0., 1.],
        ])
    }
    pub fn view(eye: Vec3, target: Vec3) -> Self {
        let f = (target - eye).normalized();
        let r = f.cross(Vec3::new(0., 1., 0.)).normalized();
        let u = r.cross(f);
        Self([
            [r.x, r.y, r.z, -r.dot(eye)],
            [u.x, u.y, u.z, -u.dot(eye)],
            [-f.x, -f.y, -f.z, f.dot(eye)],
            [0., 0., 0., 1.],
        ])
    }
    pub fn orthographic(aspect: f32, height: f32, near: f32, far: f32) -> Self {
        Self([
            [2. / (height * aspect), 0., 0., 0.],
            [0., 2. / height, 0., 0.],
            [0., 0., -2. / (far - near), -(far + near) / (far - near)],
            [0., 0., 0., 1.],
        ])
    }
    pub fn perspective(aspect: f32, fov: f32, near: f32, far: f32) -> Self {
        let q = 1. / (fov * 0.5).tan();
        Self([
            [q / aspect, 0., 0., 0.],
            [0., q, 0., 0.],
            [
                0.,
                0.,
                (far + near) / (near - far),
                2. * far * near / (near - far),
            ],
            [0., 0., -1., 0.],
        ])
    }
}
impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Self(std::array::from_fn(|r| {
            std::array::from_fn(|c| (0..4).map(|k| self.0[r][k] * b.0[k][c]).sum())
        }))
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}
