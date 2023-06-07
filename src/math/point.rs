use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Point2f {
    pub x: Float,
    pub y: Float,
}

impl Point2f {
    pub fn new(_x: Float, _y: Float) -> Point2f {
        return Point2f { x: _x, y: _y };
    }

    pub fn min(&self, rhs: &Point2f) -> Point2f {
        return Point2f {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        };
    }

    pub fn max(&self, rhs: &Point2f) -> Point2f {
        return Point2f {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        };
    }
}

#[derive(Copy, Clone)]
pub struct Point3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Point3f {
    pub fn new(_x: Float, _y: Float, _z: Float) -> Point3f {
        return Point3f {
            x: _x,
            y: _y,
            z: _z,
        };
    }
}

impl Sub<Point3f> for Point3f {
    type Output = Vector3f;

    fn sub(self, rhs: Point3f) -> Self::Output {
        return Vector3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Div<Float> for Point3f {
    type Output = Point3f;

    fn div(self, rhs: Float) -> Self::Output {
        let factor = 1.0 / rhs;

        return Point3f {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}
