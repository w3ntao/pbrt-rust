pub use crate::fundamental::color::Color;
pub use crate::fundamental::point::Point;
pub use crate::fundamental::vector3::Vector3;
pub use std::f32::consts::PI;
use std::path::Path;

pub fn get_file_name(full_path: &str) -> String {
    let file_name_with_postfix = Path::new(full_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();
    let file_name = &file_name_with_postfix[0..(&file_name_with_postfix).len() - 3];

    return file_name.to_string();
}

impl Vector3 {
    pub fn to_point(&self) -> Point {
        return Point::new(self.x, self.y, self.z);
    }
}
