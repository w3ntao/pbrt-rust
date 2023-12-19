use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGB {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        return Self { r, b, g };
    }

    pub fn black() -> Self {
        return Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    pub fn max_component(&self) -> f64 {
        return self.r.max(self.g).max(self.b);
    }

    pub fn gamma_correction(&self) -> Self {
        return Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        };
    }

    pub fn to_u256(&self) -> [u8; 3] {
        let factor = 256.0 - 0.0001;
        return [
            (self.r * factor) as u8,
            (self.g * factor) as u8,
            (self.b * factor) as u8,
        ];
    }

    pub fn clamp(&self, low: f64, high: f64) -> Self {
        let local_clamp = |x: f64| x.max(low).min(high);

        return Self {
            r: local_clamp(self.r),
            g: local_clamp(self.g),
            b: local_clamp(self.b),
        };
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
    type Output = f64;

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

impl Sub<RGB> for RGB {
    type Output = RGB;

    fn sub(self, rhs: RGB) -> Self::Output {
        return RGB {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        };
    }
}

impl Mul<f64> for RGB {
    type Output = RGB;

    fn mul(self, rhs: f64) -> Self::Output {
        return RGB {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        };
    }
}

impl Mul<RGB> for f64 {
    type Output = RGB;

    fn mul(self, rhs: RGB) -> Self::Output {
        return rhs * self;
    }
}

impl Div<f64> for RGB {
    type Output = RGB;

    fn div(self, rhs: f64) -> Self::Output {
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
