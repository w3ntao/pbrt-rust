use crate::pbrt::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        return Point2::<T> { x, y };
    }
}

impl Point2i {
    pub fn min(&self, rhs: &Point2i) -> Point2i {
        return Point2i {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        };
    }

    pub fn max(&self, rhs: &Point2i) -> Point2i {
        return Point2i {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        };
    }
}

impl Point2f {
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

impl Default for Point2f {
    fn default() -> Self {
        return Self {
            x: f64::NAN,
            y: f64::NAN,
        };
    }
}

impl From<Point2<i32>> for Point2f {
    fn from(value: Point2<i32>) -> Self {
        return Self {
            x: value.x as f64,
            y: value.y as f64,
        };
    }
}

impl<T: Display> Display for Point2<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ {}, {} ]", self.x, self.y)
    }
}

impl<T> Index<usize> for Point2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.x,
            1 => &self.y,
            _ => {
                panic!("illegal index");
            }
        };
    }
}

impl<T> IndexMut<usize> for Point2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => {
                panic!("illegal index");
            }
        };
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

impl<T: Sub<Output = T>> Sub<Point2<T>> for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Point2<T>) -> Self::Output {
        return Vector2::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl<T: Sub<Output = T>> Sub<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        return Point2::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Point2<T> {
    type Output = Point2<T>;

    fn div(self, rhs: T) -> Self::Output {
        return Self {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl Mul<Point2f> for f64 {
    type Output = Point2f;

    fn mul(self, rhs: Point2f) -> Self::Output {
        return rhs * self;
    }
}
