use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        return Vector2::<T> { x, y };
    }
}
