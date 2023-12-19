use crate::pbrt::*;

#[derive(Copy, Clone)]

pub struct CIEXYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CIEXYZ {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    pub fn from_xy_y(xy: Point2f, y: f64) -> Self {
        if xy.y == 0.0 {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        return Self::new(xy.x * y / xy.y, y, (1.0 - xy.x - xy.y) * y / xy.y);
    }

    pub fn average(&self) -> f64 {
        return (self.x + self.y + self.z) / 3.0;
    }

    pub fn xy(&self) -> Point2f {
        let sum_xyz = self.x + self.y + self.z;

        return Point2f::new(self.x / sum_xyz, self.y / sum_xyz);
    }
}

impl Index<usize> for CIEXYZ {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                unreachable!()
            }
        };
    }
}

impl Div<f64> for CIEXYZ {
    type Output = CIEXYZ;

    fn div(self, rhs: f64) -> Self::Output {
        return CIEXYZ {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}
