use rerun::external::glam::Vec3;

use geometric_algebra::simd::Simd32x3;
use geometric_algebra::*;

pub struct InertiaMap {
    pub mass: Simd32x3,
    pub inertia: Simd32x3,
}

impl InertiaMap {
    pub fn new(mass: f32, inertia: Vec3) -> Self {
        Self {
            mass: Simd32x3::from([mass, mass, mass]),
            inertia: Simd32x3::from(inertia.to_array()),
        }
    }

    pub fn rate_to_momentum(&self, line: ppga3d::Line) -> ppga3d::Line {
        ppga3d::Line::from_groups(line.group1() * self.inertia, line.group0() * self.mass)
    }

    pub fn momentum_to_rate(&self, line: ppga3d::Line) -> ppga3d::Line {
        ppga3d::Line::from_groups(line.group1() / self.mass, line.group0() / self.inertia)
    }
}
