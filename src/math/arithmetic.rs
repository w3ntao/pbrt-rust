use crate::pbrt::*;

pub const PI: Float = std::f64::consts::PI as Float;

pub fn degree_to_radian(degree: Float) -> Float {
    return (PI / 180.0) * degree;
}
