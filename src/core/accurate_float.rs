use crate::core::pbrt::*;

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

#[derive(Copy, Clone)]
pub struct ErrorFloat {
    value: f32,
    low: f32,
    high: f32,
}

impl ErrorFloat {
    pub fn without_error(v: f32) -> Self {
        return ErrorFloat {
            value: v,
            low: v,
            high: v,
        };
    }

    pub fn with_error(v: f32, error: f32) -> Self {
        return ErrorFloat {
            value: v,
            low: next_float_down(v - error),
            high: next_float_up(v + error),
        };
    }

    pub fn lower_bound(&self) -> f32 {
        return self.low;
    }

    pub fn upper_bound(&self) -> f32 {
        return self.high;
    }

    pub fn check(&self) {
        if self.low.is_finite() && self.high.is_finite() && self.high > self.low {
            return;
        }

        panic!(
            "illegal ErrorFloat: (value: {}, low: {}, high: {})",
            self.value, self.low, self.high
        );
    }
}

impl ops::Neg for ErrorFloat {
    type Output = ErrorFloat;

    fn neg(self) -> Self::Output {
        let result = ErrorFloat {
            value: -self.value,
            low: -self.high,
            high: -self.low,
        };
        result.check();

        return result;
    }
}

impl ops::Add<ErrorFloat> for ErrorFloat {
    type Output = ErrorFloat;
    fn add(self, rhs: ErrorFloat) -> ErrorFloat {
        return ErrorFloat {
            value: self.value + rhs.value,
            low: next_float_down(self.lower_bound() + rhs.lower_bound()),
            high: next_float_up(self.upper_bound() + rhs.upper_bound()),
        };
    }
}

impl ops::Sub<ErrorFloat> for ErrorFloat {
    type Output = ErrorFloat;
    fn sub(self, rhs: ErrorFloat) -> Self::Output {
        let result = ErrorFloat {
            value: self.value - rhs.value,
            low: next_float_down(self.lower_bound() - rhs.upper_bound()),
            high: next_float_up(self.upper_bound() - rhs.lower_bound()),
        };

        result.check();
        return result;
    }
}

impl ops::Mul<ErrorFloat> for ErrorFloat {
    type Output = ErrorFloat;

    fn mul(self, rhs: ErrorFloat) -> Self::Output {
        let product = [
            self.lower_bound() * rhs.lower_bound(),
            self.lower_bound() * rhs.upper_bound(),
            self.upper_bound() * rhs.lower_bound(),
            self.upper_bound() * rhs.upper_bound(),
        ];

        let result = ErrorFloat {
            value: self.value * rhs.value,
            low: product[0].min(product[1]).min(product[2].min(product[3])),
            high: product[0].max(product[1]).max(product[2].max(product[3])),
        };

        result.check();
        return result;
    }
}

impl ops::Div<ErrorFloat> for ErrorFloat {
    type Output = ErrorFloat;

    fn div(self, rhs: ErrorFloat) -> Self::Output {
        let quotient = [
            self.lower_bound() / rhs.lower_bound(),
            self.lower_bound() / rhs.upper_bound(),
            self.upper_bound() / rhs.lower_bound(),
            self.upper_bound() / rhs.upper_bound(),
        ];

        let result = ErrorFloat {
            value: self.value / rhs.value,
            low: quotient[0]
                .min(quotient[1])
                .min(quotient[2].min(quotient[3])),
            high: quotient[0]
                .max(quotient[1])
                .max(quotient[2].max(quotient[3])),
        };

        result.check();
        return result;
    }
}
