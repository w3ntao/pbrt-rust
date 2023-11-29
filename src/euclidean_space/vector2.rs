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

impl<T> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.x,
            1 => &self.x,
            _ => {
                panic!("illegal index");
            }
        };
    }
}
