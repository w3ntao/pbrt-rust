use crate::pbrt::*;

pub struct Point2<T> {
    x: T,
    y: T,
}

pub struct Point3<T> {
    x: T,
    y: T,
    z: T,
}

pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point2<T> {
    pub fn new(_x: T, _y: T) -> Point2<T> {
        return Point2 { x: _x, y: _y };
    }
}

impl<T> Point3<T> {
    pub fn new(_x: T, _y: T, _z: T) -> Point3<T> {
        return Point3::<T> {
            x: _x,
            y: _y,
            z: _z,
        };
    }
}

impl<T> Vector3<T> {
    pub fn new(_x: T, _y: T, _z: T) -> Vector3<T> {
        return Vector3::<T> {
            x: _x,
            y: _y,
            z: _z,
        };
    }
}
