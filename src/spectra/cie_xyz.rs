use crate::pbrt::*;

pub struct CIEXYZ {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl CIEXYZ {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        return Self { x, y, z };
    }

    pub fn from_xy_y(xy: Point2f, y: Float) -> Self {
        if xy.y == 0.0 {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        return Self::new(xy.x * y / xy.y, y, (1.0 - xy.x - xy.y) * y / xy.y);
    }

    pub fn average(&self) -> Float {
        return (self.x + self.y + self.z) / 3.0;
    }

    pub fn xy(&self) -> Point2f {
        let sum_xyz = self.x + self.y + self.z;

        return Point2f::new(self.x / sum_xyz, self.y / sum_xyz);
    }
}

impl Div<Float> for CIEXYZ {
    type Output = CIEXYZ;

    fn div(self, rhs: Float) -> Self::Output {
        return CIEXYZ {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}
