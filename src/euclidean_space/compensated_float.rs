use crate::pbrt::*;

pub struct CompensatedFloat {
    pub value: f64,
    pub error: f64,
}

impl CompensatedFloat {
    pub fn new(_value: f64, _error: f64) -> Self {
        return CompensatedFloat {
            value: _value,
            error: _error,
        };
    }

    pub fn eval(&self) -> f64 {
        return self.value + self.error;
    }
}

pub fn two_prod(a: f64, b: f64) -> CompensatedFloat {
    let ab = a * b;
    return CompensatedFloat::new(ab, fma(a, b, -ab));
}

pub fn two_sum(a: f64, b: f64) -> CompensatedFloat {
    let s = a + b;
    let delta = s - a;
    return CompensatedFloat::new(s, (a - (s - delta)) + (b - delta));
}

pub fn inner_product<const N: usize>(left: &[f64; N], right: &[f64; N]) -> CompensatedFloat {
    fn _inner_product(left: &[f64], right: &[f64], n: usize) -> CompensatedFloat {
        let ab = two_prod(left[0], right[0]);
        if n == 1 {
            return ab;
        }

        let tp = _inner_product(&left[1..], &right[1..], n - 1);
        let sum = two_sum(ab.value, tp.value);

        return CompensatedFloat::new(sum.value, ab.error + (tp.error + sum.error));
    }

    return _inner_product(left, right, N);
}
