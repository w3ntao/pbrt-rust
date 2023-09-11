use crate::pbrt::*;
pub const PI: Float = std::f64::consts::PI as Float;

pub const PI_OVER_2: Float = PI / 2.0;

pub const PI_OVER_4: Float = PI / 4.0;

pub const INV_PI: Float = 1.0 / PI;

pub fn degree_to_radian(degree: Float) -> Float {
    return (PI / 180.0) * degree;
}

pub const fn lerp(x: Float, a: Float, b: Float) -> Float {
    return (1.0 - x) * a + x * b;
}

pub const fn clamp(val: Float, low: Float, high: Float) -> Float {
    if val < low {
        return low;
    }
    if val > high {
        return high;
    }
    return val;
}

pub const fn usize_clamp(val: usize, low: usize, high: usize) -> usize {
    if val < low {
        return low;
    }
    if val > high {
        return high;
    }
    return val;
}

pub fn template_clamp<T: PartialOrd>(val: T, low: T, high: T) -> T {
    if val < low {
        return low;
    }
    if val > high {
        return high;
    }
    return val;
}
