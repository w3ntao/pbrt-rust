use rand_distr::num_traits::Zero;

pub fn next_float_up(x: f32) -> f32 {
    if x.is_infinite() || x.is_zero() {
        return x;
    }

    let bits = x.to_bits();
    return f32::from_bits(if x >= 0.0 { bits + 1 } else { bits - 1 });
}

pub fn next_float_down(x: f32) -> f32 {
    if x.is_infinite() || x.is_zero() {
        return x;
    }

    let bits = x.to_bits();
    return f32::from_bits(if x >= 0.0 { bits - 1 } else { bits + 1 });
}

const MACHINE_EPSILON: f32 = f32::EPSILON * 0.5;

pub fn gamma(n: u32) -> f32 {
    let float_n = n as f32;
    return (float_n * MACHINE_EPSILON) / (1.0 - float_n * MACHINE_EPSILON);
}
