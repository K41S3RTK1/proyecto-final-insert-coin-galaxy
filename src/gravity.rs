//! Softened inverse-square planetary gravity. Distances are world metres.
use crate::math::Vec3;

pub fn acceleration(ship: Vec3, centers: impl IntoIterator<Item = Vec3>) -> Vec3 {
    let mut sum = Vec3::default();
    for center in centers {
        let delta = center - ship;
        let r = delta.length();
        // Smoothly fade between 30 and 40 m, avoiding discontinuities on entry.
        let fade = ((40. - r) / 10.).clamp(0., 1.);
        let fade = fade * fade * (3. - 2. * fade);
        // Softening prevents a singularity even inside the collision sphere.
        sum = sum + delta * (180. * fade / (r * r + 9.).powf(1.5));
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gravity_is_attractive_finite_and_has_smooth_range() {
        let center = Vec3::default();
        let near = acceleration(Vec3::new(10., 0., 0.), [center]);
        let far = acceleration(Vec3::new(20., 0., 0.), [center]);
        assert!(near.x < far.x && far.x < 0.);
        assert_eq!(near.y, 0.);
        assert_eq!(acceleration(center, [center]).length(), 0.);
        assert_eq!(acceleration(Vec3::new(40., 0., 0.), [center]).length(), 0.);
        assert!(acceleration(Vec3::new(39.99, 0., 0.), [center]).length() < 1e-5);
        let symmetric = acceleration(center, [Vec3::new(15., 0., 0.), Vec3::new(-15., 0., 0.)]);
        assert!(symmetric.length() < 1e-6);
    }
}
