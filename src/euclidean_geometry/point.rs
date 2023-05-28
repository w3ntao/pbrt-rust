use crate::pbrt::*;

pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(_x: T, _y: T) -> Point2<T> {
        return Point2 { x: _x, y: _y };
    }
}

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
