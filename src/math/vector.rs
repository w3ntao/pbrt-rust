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

impl<T: Numerical> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        return Self { x, y, z };
    }
}

impl Vector3<Float> {
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

impl<T: Numerical> From<Point3<T>> for Vector3<T> {
    fn from(p: Point3<T>) -> Self {
        return Self {
            x: p.x,
            y: p.y,
            z: p.z,
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
