use crate::math::point::Point3;
use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Vector2<T: Numerical> {
    pub x: T,
    pub y: T,
}

impl<T: Numerical> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        return Vector2::<T> { x, y };
    }
}

#[derive(Copy, Clone)]
pub struct Vector3<T: Numerical> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Numerical> From<Point3<T>> for Vector3<T> {
    fn from(p: Point3<T>) -> Self {
        return Self {
            x: p.x,
            y: p.y,
            z: p.z,
        };
    }
}

impl From<Vector3<Float>> for Vector3<Interval> {
    fn from(value: Vector3<Float>) -> Self {
        return Vector3::<Interval>::new(
            Interval::from(value.x),
            Interval::from(value.y),
            Interval::from(value.z),
        );
    }
}

impl From<Point3<Float>> for Vector3<Interval> {
    fn from(value: Point3<Float>) -> Self {
        return Vector3::<Interval>::new(
            Interval::from(value.x),
            Interval::from(value.y),
            Interval::from(value.z),
        );
    }
}

impl<T: Numerical> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        return Self { x, y, z };
    }
}

impl Vector3<Float> {
    pub fn abs(&self) -> Vector3<Float> {
        return Vector3::<Float> {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        };
    }

    pub fn dot(&self, v: Vector3<Float>) -> Float {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn length_squared(&self) -> Float {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> Float {
        return Float::sqrt(self.length_squared());
    }

    pub fn normalize(&self) -> Vector3f {
        return *self / self.length();
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        return Self {
            x: difference_of_products(self.y, rhs.z, self.z, rhs.y),
            y: difference_of_products(self.z, rhs.x, self.x, rhs.z),
            z: difference_of_products(self.x, rhs.y, self.y, rhs.x),
        };
    }
}

impl Vector3<Interval> {
    pub fn new_with_error(v: Vector3f, error: Vector3f) -> Self {
        return Vector3fi {
            x: Interval::from_value_and_error(v.x, error.x),
            y: Interval::from_value_and_error(v.y, error.y),
            z: Interval::from_value_and_error(v.z, error.z),
        };
    }

    pub fn is_exact(&self) -> bool {
        return self.x.width() == 0.0 && self.y.width() == 0.0 && self.z.width() == 0.0;
    }

    pub fn error(&self) -> Vector3<Float> {
        return Vector3::<Float> {
            x: self.x.width() / 2.0,
            y: self.y.width() / 2.0,
            z: self.z.width() / 2.0,
        };
    }
}

impl<T: Numerical + Mul<Float, Output = T>> Mul<Float> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Float) -> Self::Output {
        return Vector3::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Div<Float> for Vector3<Float> {
    type Output = Vector3<Float>;

    fn div(self, rhs: Float) -> Self::Output {
        let one_over_rhs = 1.0 / rhs;
        return Vector3::<Float> {
            x: self.x * one_over_rhs,
            y: self.y * one_over_rhs,
            z: self.z * one_over_rhs,
        };
    }
}
