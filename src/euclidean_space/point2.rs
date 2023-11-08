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

impl Default for Point2<Float> {
    fn default() -> Self {
        return Self {
            x: Float::NAN,
            y: Float::NAN,
        };
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

impl<T: Display> Display for Point2<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ {}, {} ]", self.x, self.y)
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

impl Mul<Point2f> for Float {
    type Output = Point2f;

    fn mul(self, rhs: Point2f) -> Self::Output {
        return rhs * self;
    }
}
