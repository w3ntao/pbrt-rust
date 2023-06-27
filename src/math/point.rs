use crate::math::vector::*;
use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Point2<T: Numerical> {
    pub x: T,
    pub y: T,
}

impl<T: Numerical> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        return Point2::<T> { x, y };
    }
}

impl From<Point2<i32>> for Point2<Float> {
    fn from(value: Point2<i32>) -> Self {
        return Self {
            x: value.x as Float,
            y: value.y as Float,
        };
    }
}

#[derive(Copy, Clone)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl From<Point3<Float>> for Point3<Interval> {
    fn from(value: Point3<Float>) -> Self {
        return Point3::<Interval>::new(
            Interval::new(value.x),
            Interval::new(value.y),
            Interval::new(value.z),
        );
    }
}

impl From<Vector3<Interval>> for Point3<Interval> {
    fn from(value: Vector3<Interval>) -> Self {
        return Point3::<Interval>::new(value.x, value.y, value.z);
    }
}

impl Point2<Float> {
    pub fn min(&self, rhs: &Point2<Float>) -> Point2<Float> {
        return Point2::<Float> {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        };
    }

    pub fn max(&self, rhs: &Point2<Float>) -> Point2<Float> {
        return Point2::<Float> {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        };
    }
}

impl<T: Numerical + Display> Display for Point2<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<Output = T> + Numerical> Add<Point2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, rhs: Point2<T>) -> Self::Output {
        return Point2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: Add<Output = T> + Numerical> Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        return Point2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        return Point3::<T> { x, y, z };
    }
}

impl<T: Add<Output = T>> Add<Point3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Point3<T>) -> Self::Output {
        return Point3::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl<T: Sub<Output = T> + Numerical> Sub<Point3<T>> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Point3<T>) -> Self::Output {
        return Vector3::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Div<Float> for Point3<Float> {
    type Output = Point3<Float>;

    fn div(self, rhs: Float) -> Self::Output {
        let factor = 1.0 / rhs;
        return Point3::<Float> {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}

impl<T: Display> Display for Point3<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
