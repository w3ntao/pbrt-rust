use rand_distr::num_traits::Zero;

pub fn next_float_up(x: f32) -> f32 {
    if x.is_infinite() || x.is_zero() {
        return x;
    }

    let bits = x.to_bits();
    return f32::from_bits(if x >= 0.0 { bits + 1 } else { bits - 1 });
}
