use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct RGBColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl RGBColor {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        return Self { r, b, g };
    }
    pub fn black() -> Self {
        return Self {
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

impl Add<RGBColor> for RGBColor {
    type Output = RGBColor;

    fn add(self, rhs: RGBColor) -> Self::Output {
        return RGBColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl Div<Float> for RGBColor {
    type Output = RGBColor;

    fn div(self, rhs: Float) -> Self::Output {
        return RGBColor {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        };
    }
}

impl AddAssign<RGBColor> for RGBColor {
    fn add_assign(&mut self, rhs: RGBColor) {
        *self = *self + rhs;
    }
}
