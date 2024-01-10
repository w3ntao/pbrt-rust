use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<Point2<T>> for Vector2<T> {
    fn from(value: Point2<T>) -> Self {
        return Self {
            x: value.x,
            y: value.y,
        };
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        return Vector2::<T> { x, y };
    }

    pub fn length_squared(&self) -> T {
        return sqr(self.x) + sqr(self.y);
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
