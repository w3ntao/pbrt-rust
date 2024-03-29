use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Default for Point3<f64> {
    fn default() -> Self {
        return Self {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        };
    }
}

impl From<Point3<f64>> for Point3<Interval> {
    fn from(value: Point3<f64>) -> Self {
        return Point3::<Interval>::new(
            Interval::from(value.x),
            Interval::from(value.y),
            Interval::from(value.z),
        );
    }
}

impl From<Point3<Interval>> for Point3<f64> {
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

impl<T> IndexMut<usize> for Point3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
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

impl Mul<f64> for Point3<f64> {
    type Output = Point3<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        return Point3::<f64> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Mul<Point3<f64>> for f64 {
    type Output = Point3<f64>;

    fn mul(self, rhs: Point3<f64>) -> Self::Output {
        return rhs * self;
    }
}

impl MulAssign<f64> for Point3<f64> {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Point3<f64> {
    type Output = Point3<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let factor = 1.0 / rhs;
        return Point3::<f64> {
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

impl Point3<f64> {
    pub fn nan() -> Self {
        return Self {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        };
    }
    pub fn min(&self, p: Self) -> Self {
        return Self {
            x: self.x.min(p.x),
            y: self.y.min(p.y),
            z: self.z.min(p.z),
        };
    }

    pub fn max(&self, p: Self) -> Self {
        return Self {
            x: self.x.max(p.x),
            y: self.y.max(p.y),
            z: self.z.max(p.z),
        };
    }

    pub fn abs(&self) -> Self {
        return Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        };
    }
}

impl Point3<Interval> {
    pub fn from_value_and_error(p: Point3<f64>, e: Vector3<f64>) -> Self {
        return Self {
            x: Interval::from_value_and_error(p.x, e.x),
            y: Interval::from_value_and_error(p.y, e.y),
            z: Interval::from_value_and_error(p.z, e.z),
        };
    }
    pub fn error(&self) -> Vector3<f64> {
        return Vector3::<f64> {
            x: self.x.width() / 2.0,
            y: self.y.width() / 2.0,
            z: self.z.width() / 2.0,
        };
    }

    pub fn is_exact(&self) -> bool {
        return self.x.width() == 0.0 && self.y.width() == 0.0 && self.z.width() == 0.0;
    }
}

impl Div<f64> for Point3<Interval> {
    type Output = Point3<Interval>;

    fn div(self, rhs: f64) -> Self::Output {
        return Point3::<Interval> {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}
