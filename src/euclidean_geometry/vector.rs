use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Vector3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector3f {
    pub fn new(_x: Float, _y: Float, _z: Float) -> Vector3f {
        return Vector3f {
            x: _x,
            y: _y,
            z: _z,
        };
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

    pub fn cross(&self, rhs: &Vector3f) -> Vector3f {
        return Vector3f::new(
            difference_of_products(self.y, rhs.z, self.z, rhs.y),
            difference_of_products(self.z, rhs.x, self.x, rhs.z),
            difference_of_products(self.x, rhs.y, self.y, rhs.x),
        );
    }
}

impl Index<usize> for Vector3f {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        return {
            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                _ => {
                    panic!("illegal indexing: {}", index);
                }
            }
        };
    }
}

impl Div<Float> for Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: Float) -> Self::Output {
        let one_over_rhs = 1.0 / rhs;
        return Vector3f {
            x: self.x * one_over_rhs,
            y: self.y * one_over_rhs,
            z: self.z * one_over_rhs,
        };
    }
}
