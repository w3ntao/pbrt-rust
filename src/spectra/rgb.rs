use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct RGB {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl RGB {
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

    pub fn max_component(&self) -> Float {
        return self.r.max(self.g).max(self.b);
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ RGB [{}, {}, {}] ]", self.r, self.g, self.b)
    }
}

impl From<Vector3f> for RGB {
    fn from(value: Vector3f) -> Self {
        return RGB {
            r: value.x,
            g: value.y,
            b: value.z,
        };
    }
}

impl Index<usize> for RGB {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => {
                unreachable!();
            }
        };
    }
}

impl Add<RGB> for RGB {
    type Output = RGB;

    fn add(self, rhs: RGB) -> Self::Output {
        return RGB {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl Mul<Float> for RGB {
    type Output = RGB;

    fn mul(self, rhs: Float) -> Self::Output {
        return RGB {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        };
    }
}

impl Mul<RGB> for Float {
    type Output = RGB;

    fn mul(self, rhs: RGB) -> Self::Output {
        return rhs * self;
    }
}

impl Div<Float> for RGB {
    type Output = RGB;

    fn div(self, rhs: Float) -> Self::Output {
        return RGB {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        };
    }
}

impl AddAssign<RGB> for RGB {
    fn add_assign(&mut self, rhs: RGB) {
        *self = *self + rhs;
    }
}