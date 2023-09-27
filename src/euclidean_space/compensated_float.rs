use crate::pbrt::*;

pub struct CompensatedFloat {
    pub value: Float,
    pub error: Float,
}

impl CompensatedFloat {
    pub fn new(_value: Float, _error: Float) -> Self {
        return CompensatedFloat {
            value: _value,
            error: _error,
        };
    }

    pub fn eval(&self) -> Float {
        return self.value + self.error;
    }
}

pub fn two_prod(a: Float, b: Float) -> CompensatedFloat {
    let ab = a * b;
    return CompensatedFloat::new(ab, fma(a, b, -ab));
}

pub fn two_sum(a: Float, b: Float) -> CompensatedFloat {
    let s = a + b;
    let delta = s - a;
    return CompensatedFloat::new(s, (a - (s - delta)) + (b - delta));
}

pub fn inner_product(left: &[Float], right: &[Float]) -> CompensatedFloat {
    // TODO: rewrite this with template
    let length = left.len();
    assert_eq!(length, right.len());

    fn _inner_product(left: &[Float], right: &[Float]) -> CompensatedFloat {
        let ab = two_prod(left[0], right[0]);
        if left.len() == 1 {
            return ab;
        }

        let tp = _inner_product(&left[1..], &right[1..]);
        let sum = two_sum(ab.value, tp.value);

        return CompensatedFloat::new(sum.value, ab.error + (tp.error + sum.error));
    }

    return _inner_product(left, right);
}
