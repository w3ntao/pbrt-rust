use crate::pbrt::*;
use std::ffi::c_float;

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

pub const fn clamp_float(val: Float, low: Float, high: Float) -> Float {
    if val < low {
        return low;
    }
    if val > high {
        return high;
    }
    return val;
}

pub const fn clamp_usize(val: usize, low: usize, high: usize) -> usize {
    if val < low {
        return low;
    }
    if val > high {
        return high;
    }
    return val;
}

pub fn sigmoid(x: Float) -> Float {
    if x.is_infinite() {
        return if x > 0.0 { 1.0 } else { 0.0 };
    }

    return 0.5 + x / (2.0 * (1.0 + x * x).sqrt());
}

pub fn evaluate_polynomial(t: Float, args: &[Float]) -> Float {
    let c = args[0];
    let length = args.len();

    if length == 1 {
        return c;
    }

    return fma(t, evaluate_polynomial(t, &args[1..length]), c);
}

pub fn difference_of_products(a: Float, b: Float, c: Float, d: Float) -> Float {
    let cd = c * d;
    let difference_of_products = fma(a, b, -cd);
    let error = fma(-c, d, cd);
    return difference_of_products + error;
}

pub const fn is_power_of_2(v: i32) -> bool {
    return v > 0 && !(v & (v - 1) > 0);
}

pub const fn round_up_pow_2(v: i32) -> i32 {
    let mut x = v - 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;

    return x + 1;
}

// http://www.plunk.org/~hatch/rightway.html
pub fn sinx_over_x(x: Float) -> Float {
    if 1.0 - x * x == 1.0 {
        return 1.0;
    }

    return x.sin() / x;
}

pub fn sinc(x: Float) -> Float {
    return sinx_over_x(PI * x);
}

pub fn windowed_sinc(x: Float, radius: Float, tau: Float) -> Float {
    if x.abs() > radius {
        return 0.0;
    }
    return sinc(x) * sinc(x / tau);
}
