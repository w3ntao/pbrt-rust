pub use crate::fundamental::color::Color;
pub use crate::fundamental::point::Point;
pub use crate::fundamental::vector3::Vector3;
pub use std::f32::consts::PI;

impl Vector3 {
    pub fn to_point(&self) -> Point {
        return Point::new(self.x, self.y, self.z);
    }
}
