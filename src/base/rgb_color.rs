use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct RGBColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl RGBColor {
    pub fn black() -> RGBColor {
        return RGBColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }
}

impl From<Vector3f> for RGBColor {
    fn from(value: Vector3f) -> Self {
        return RGBColor {
            r: value.x,
            g: value.y,
            b: value.z,
        };
    }
}
