pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
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
