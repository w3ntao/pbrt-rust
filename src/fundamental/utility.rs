use std::path::Path;

use rand::distributions::Uniform;
use rand::thread_rng;
use rand_distr::Distribution;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::fundamental::vector3::Vector3;

pub fn get_file_name(full_path: &str) -> String {
    let file_name_with_postfix = Path::new(full_path).file_name().and_then(|s| s.to_str()).unwrap();
    let file_name = &file_name_with_postfix[0..(&file_name_with_postfix).len() - 3];

    return file_name.to_string();
}

pub fn random_in_range(low: f32, high: f32) -> f32 {
    let mut rng = thread_rng();
    let uniform_distribution = Uniform::new(low, high);
    return uniform_distribution.sample(&mut rng);
}

pub fn random_zero_to_one() -> f32 {
    return random_in_range(0.0, 1.0);
}

pub fn random_color() -> Color {
    return Color::new(random_zero_to_one(), random_zero_to_one(), random_zero_to_one());
}

pub fn to_vector3(p: Point) -> Vector3 {
    return Vector3::new(p.x, p.y, p.z);
}

pub fn to_point(v: Vector3) -> Point {
    return Point::new(v.x, v.y, v.z);
}
