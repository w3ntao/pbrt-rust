use crate::pbrt::*;

pub const PI: f64 = std::f64::consts::PI as f64;

pub const PI_OVER_2: f64 = PI / 2.0;

pub const PI_OVER_4: f64 = PI / 4.0;

pub const INV_PI: f64 = 1.0 / PI;

pub fn degree_to_radian(degree: f64) -> f64 {
    return (PI / 180.0) * degree;
}

pub const fn lerp(x: f64, a: f64, b: f64) -> f64 {
    return (1.0 - x) * a + x * b;
}

pub fn mod_i32(a: i32, b: i32) -> i32 {
    let result = a - (a / b) * b;
    return if result < 0 { result + b } else { result };
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

pub fn sigmoid(x: f64) -> f64 {
    if x.is_infinite() {
        return if x > 0.0 { 1.0 } else { 0.0 };
    }

    return 0.5 + x / (2.0 * (1.0 + x * x).sqrt());
}

pub fn evaluate_polynomial(t: f64, args: &[f64]) -> f64 {
    let c = args[0];
    let length = args.len();

    if length == 1 {
        return c;
    }

    return fma(t, evaluate_polynomial(t, &args[1..length]), c);
}

pub fn difference_of_products(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let cd = c * d;
    let difference_of_prod = fma(a, b, -cd);
    let error = fma(-c, d, cd);

    return difference_of_prod + error;
}

fn fma_f_v3_v3(a: f64, b: Vector3f, c: Vector3f) -> Vector3f {
    return Vector3f {
        x: fma(a, b.x, c.x),
        y: fma(a, b.y, c.y),
        z: fma(a, b.z, c.z),
    };
}

pub fn difference_of_products_vec3(a: f64, b: Vector3f, c: f64, d: Vector3f) -> Vector3f {
    let cd = c * d;
    let difference_of_prod = fma_f_v3_v3(a, b, -cd);
    let error = fma_f_v3_v3(-c, d, cd);

    return difference_of_prod + error;
}

pub fn sum_of_products(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let cd = c * d;
    let sum = fma(a, b, cd);
    let error = fma(c, d, -cd);
    return sum + error;
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

pub fn safe_asin(x: f64) -> f64 {
    debug_assert!(x >= -1.0001 && x <= 1.0001);

    return x.clamp(-1.0, 1.0).asin();
}

pub fn safe_acos(x: f64) -> f64 {
    debug_assert!(x >= -1.0001 && x <= 1.0001);

    return x.clamp(-1.0, 1.0).acos();
}

pub fn gram_schmidt(v: Vector3f, w: Vector3f) -> Vector3f {
    return v - v.dot(w) * w;
}

pub fn sqr<T: Mul<Output = T> + Copy>(x: T) -> T {
    return x * x;
}

pub fn safe_sqrt(x: f64) -> f64 {
    debug_assert!(x >= -1e-3);
    return x.max(0.0).sqrt();
}

// http://www.plunk.org/~hatch/rightway.html
pub fn sinx_over_x(x: f64) -> f64 {
    if 1.0 - x * x == 1.0 {
        return 1.0;
    }

    return x.sin() / x;
}

pub fn sinc(x: f64) -> f64 {
    return sinx_over_x(PI * x);
}

pub fn windowed_sinc(x: f64, radius: f64, tau: f64) -> f64 {
    if x.abs() > radius {
        return 0.0;
    }
    return sinc(x) * sinc(x / tau);
}

pub fn spherical_triangle_area(a: Vector3f, b: Vector3f, c: Vector3f) -> f64 {
    return a
        .dot(b.cross(c))
        .atan2(1.0 + a.dot(b) + a.dot(c) + b.dot(c))
        .abs()
        * 2.0;
}
