use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
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
            Interval::from(value.x),
            Interval::from(value.y),
            Interval::from(value.z),
        );
    }
}

impl From<Point3<Interval>> for Point3<Float> {
    fn from(value: Point3<Interval>) -> Self {
        return Point3 {
            x: value.x.midpoint(),
            y: value.y.midpoint(),
            z: value.z.midpoint(),
        };
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

impl<T: Display> Display for Point2<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add<Point2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, rhs: Point2<T>) -> Self::Output {
        return Point2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: Add<Output = T>> Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        return Point2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T> Index<usize> for Point3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Point3: illegal index: {}", index);
            }
        };
    }
}

impl<T: Add<Output = T>> Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        return Point3::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
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

impl<T: Add<Output = T> + Copy> AddAssign<Point3<T>> for Point3<T> {
    fn add_assign(&mut self, rhs: Point3<T>) {
        *self = *self + rhs;
    }
}

impl<T: Sub<Output = T>> Sub<Point3<T>> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Point3<T>) -> Self::Output {
        return Vector3::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl<T: Sub<Output = T>> Sub<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        return Point3::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul<Float> for Point3<Float> {
    type Output = Point3<Float>;

    fn mul(self, rhs: Float) -> Self::Output {
        return Point3::<Float> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Mul<Point3<Float>> for Float {
    type Output = Point3<Float>;

    fn mul(self, rhs: Point3<Float>) -> Self::Output {
        return rhs * self;
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
        write!(f, "[ {}, {}, {} ]", self.x, self.y, self.z)
    }
}

impl<T: Copy> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        return Point3::<T> { x, y, z };
    }

    pub fn permute(&self, p: [usize; 3]) -> Point3<T> {
        return Point3::<T> {
            x: self[p[0]],
            y: self[p[1]],
            z: self[p[2]],
        };
    }
}

impl Point3<Float> {
    pub fn min(&self, p: Point3<Float>) -> Point3<Float> {
        return Point3::<Float> {
            x: self.x.min(p.x),
            y: self.y.min(p.y),
            z: self.z.min(p.z),
        };
    }

    pub fn max(&self, p: Point3<Float>) -> Point3<Float> {
        return Point3::<Float> {
            x: self.x.max(p.x),
            y: self.y.max(p.y),
            z: self.z.max(p.z),
        };
    }
}

impl Point3<Interval> {
    pub fn error(&self) -> Vector3<Float> {
        return Vector3::<Float> {
            x: self.x.width() / 2.0,
            y: self.y.width() / 2.0,
            z: self.z.width() / 2.0,
        };
    }
}
