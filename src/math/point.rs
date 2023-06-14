use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Point2i {
    pub x: i32,
    pub y: i32,
}

impl Point2i {
    pub fn new(_x: i32, _y: i32) -> Point2i {
        return Point2i { x: _x, y: _y };
    }
}

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

impl std::fmt::Display for Point2f {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
pub struct Point3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl std::fmt::Display for Point3f {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
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
