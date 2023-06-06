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

pub fn difference_of_products(a: Float, b: Float, c: Float, d: Float) -> Float {
    let cd = c * d;
    let difference_of_products = fma(a, b, -cd);
    let error = fma(-c, d, cd);
    return difference_of_products + error;
}
