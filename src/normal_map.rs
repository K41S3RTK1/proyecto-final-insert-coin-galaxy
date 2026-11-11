//! Tangent-space normal texture: decode RGB, bilinear repeat sampling, TBN to world.
use crate::math::Vec3;
use std::{
    f32::consts::{PI, TAU},
    sync::OnceLock,
};

pub struct NormalMap {
    width: usize,
    height: usize,
    pixels: Vec<Vec3>,
}
impl NormalMap {
    pub fn terrain() -> &'static Self {
        static MAP: OnceLock<NormalMap> = OnceLock::new();
        MAP.get_or_init(|| {
            let image =
                image::load_from_memory(include_bytes!("../assets/textures/terrain-normal.png"))
                    .expect("bundled normal map must be a valid PNG")
                    .to_rgb8();
            Self {
                width: image.width() as usize,
                height: image.height() as usize,
                pixels: image
                    .pixels()
                    .map(|p| {
                        Vec3::new(
                            p[0] as f32 / 127.5 - 1.,
                            p[1] as f32 / 127.5 - 1.,
                            p[2] as f32 / 127.5 - 1.,
                        )
                    })
                    .collect(),
            }
        })
    }
    pub fn sample(&self, u: f32, v: f32) -> Vec3 {
        let x = u.rem_euclid(1.) * self.width as f32 - 0.5;
        let y = v.rem_euclid(1.) * self.height as f32 - 0.5;
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let fetch = |dx: i32, dy: i32| {
            self.pixels[(y0 + dy).rem_euclid(self.height as i32) as usize * self.width
                + (x0 + dx).rem_euclid(self.width as i32) as usize]
        };
        fetch(0, 0)
            .lerp(fetch(1, 0), x - x.floor())
            .lerp(fetch(0, 1).lerp(fetch(1, 1), x - x.floor()), y - y.floor())
            .normalized()
    }
}
pub fn tangent(p: Vec3) -> Vec3 {
    let t = Vec3::new(-p.z, 0., p.x);
    if t.length() < 1e-6 {
        Vec3::new(1., 0., 0.)
    } else {
        t.normalized()
    }
}
pub fn uv(p: Vec3) -> (f32, f32) {
    let p = p.normalized();
    (
        (p.z.atan2(p.x) / TAU + 0.5) * 4.,
        (0.5 - p.y.clamp(-1., 1.).asin() / PI) * 2.,
    )
}
pub fn transform(n: Vec3, tangent: Vec3, sample: Vec3, strength: f32) -> Vec3 {
    if strength == 0. {
        return n;
    }
    let t = tangent - n * n.dot(tangent);
    let t = if t.length() < 1e-6 {
        let axis = if n.y.abs() < 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        axis.cross(n).normalized()
    } else {
        t.normalized()
    };
    let b = n.cross(t);
    (t * (sample.x * strength) + b * (sample.y * strength) + n * sample.z).normalized()
}
pub fn mapped(n: Vec3, t: Vec3, p: Vec3) -> Vec3 {
    let (u, v) = uv(p);
    transform(n, t, NormalMap::terrain().sample(u, v), 0.8)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flat_normal_and_rotated_tbn_preserve_expected_directions() {
        let n = Vec3::new(0., 0., 1.);
        let t = Vec3::new(1., 0., 0.);
        assert!((transform(n, t, Vec3::new(0., 0., 1.), 1.) - n).length() < 1e-6);
        let sample = Vec3::new(0.5, -0.2, 1.).normalized();
        let expected = transform(n, t, sample, 0.8).rotate_y(1.3);
        assert!(
            (transform(n.rotate_y(1.3), t.rotate_y(1.3), sample, 0.8) - expected).length() < 1e-5
        );
        assert_eq!(transform(n, t, sample, 0.).z, 1.);
    }
    #[test]
    fn map_repeats_and_remains_finite_at_poles() {
        let map = NormalMap::terrain();
        assert!((map.sample(0.13, 0.27) - map.sample(1.13, -0.73)).length() < 1e-4);
        assert!((map.sample(0.13, 0.27) - map.sample(0.38, 0.51)).length() > 0.01);
        for p in [Vec3::new(0., 1., 0.), Vec3::new(0., -1., 0.)] {
            assert!((mapped(p, tangent(p), p).length() - 1.).abs() < 1e-5);
        }
    }
}
