use geometric_algebra::*;
use rerun::external::glam;

pub(crate) trait ToGlam {
    type Output;
    fn to_glam(&self) -> Self::Output;
}

impl ToGlam for ppga3d::Point {
    type Output = glam::Vec3;
    fn to_glam(&self) -> glam::Vec3 {
        let g = self.group0();
        glam::Vec3::new(g[1], g[2], g[3]) / g[0]
    }
}

pub(crate) trait ToPpgaPoint {
    type Output;
    fn to_ppga_point(&self) -> Self::Output;
}

impl ToPpgaPoint for glam::Vec3 {
    type Output = ppga3d::Point;
    fn to_ppga_point(&self) -> ppga3d::Point {
        ppga3d::Point::new(1.0, self.x, self.y, self.z)
    }
}

pub(crate) trait ToPpgaTranslator {
    type Output;
    fn to_ppga_translator(&self) -> Self::Output;
}

impl ToPpgaTranslator for glam::Vec3 {
    type Output = ppga3d::Translator;
    fn to_ppga_translator(&self) -> ppga3d::Translator {
        ppga3d::Translator::new(1.0, self.x * -0.5, self.y * -0.5, self.z * -0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ORIGIN;

    #[test]
    fn glam_vec_as_translator() {
        let v = glam::vec3(1.0, 2.0, 3.0);
        let t = v.to_ppga_translator();
        let p = t.transformation(ORIGIN);
        let p = p.to_glam();
        assert_eq!(p, v);
    }
}
