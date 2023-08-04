use crate::pbrt::*;

const MACHINE_EPSILON: Float = Float::EPSILON * 0.5;

const fn compute_gamma(n: usize) -> Float {
    let float_n = n as Float;
    return (float_n * MACHINE_EPSILON) / (1.0 - float_n * MACHINE_EPSILON);
}

const PRECOMPUTED_GAMMA: [Float; 128] = {
    let mut seq = [Float::NAN; 128];
    let mut i = 0;
    while i < 128 {
        seq[i] = compute_gamma(i);
        i += 1;
    }
    seq
};

pub const fn gamma(n: usize) -> Float {
    return PRECOMPUTED_GAMMA[n];
}

pub fn next_float_up(v: Float) -> Float {
    if v.is_infinite() && v > 0.0 {
        return v;
    }

    let adjusted_v = if v == -0.0 { 0.0 } else { v };
    let bits = adjusted_v.to_bits();

    return Float::from_bits(if adjusted_v >= 0.0 {
        bits + 1
    } else {
        bits - 1
    });
}

pub fn next_float_down(v: Float) -> Float {
    if v.is_infinite() && v < 0.0 {
        return v;
    }

    let adjusted_v = if v == 0.0 { -0.0 } else { v };
    let bits = adjusted_v.to_bits();

    return Float::from_bits(if adjusted_v > 0.0 { bits - 1 } else { bits + 1 });
}

pub fn add_round_up(a: Float, b: Float) -> Float {
    return next_float_up(a + b);
}

pub fn add_round_down(a: Float, b: Float) -> Float {
    return next_float_down(a + b);
}

pub fn sub_round_up(a: Float, b: Float) -> Float {
    return add_round_up(a, -b);
}

pub fn sub_round_down(a: Float, b: Float) -> Float {
    return add_round_down(a, -b);
}

pub fn mul_round_up(a: Float, b: Float) -> Float {
    return next_float_up(a * b);
}

pub fn mul_round_down(a: Float, b: Float) -> Float {
    return next_float_down(a * b);
}

pub fn div_round_up(a: Float, b: Float) -> Float {
    return next_float_up(a / b);
}

pub fn div_round_down(a: Float, b: Float) -> Float {
    return next_float_down(a / b);
}

pub fn sqrt_round_up(a: Float) -> Float {
    return next_float_up(a.sqrt());
}

pub fn sqrt_round_down(a: Float) -> Float {
    return next_float_down(a.sqrt()).max(0.0);
}
