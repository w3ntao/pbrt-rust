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
    v: f32,
    low: f32,
    high: f32,
}

impl ErrorFloat {
    pub fn without_error(_v: f32) -> Self {
        return ErrorFloat {
            v: _v,
            low: _v,
            high: _v,
        };
    }

    pub fn with_error(_v: f32, error: f32) -> Self {
        return ErrorFloat {
            v: _v,
            low: next_float_down(_v - error),
            high: next_float_up(_v + error),
        };
    }

    pub fn value(&self) -> f32 {
        return self.v;
    }

    pub fn lower_bound(&self) -> f32 {
        return self.low;
    }

    pub fn upper_bound(&self) -> f32 {
        return self.high;
    }

    pub fn check(&self) {
        if self.v.is_finite()
            && self.low.is_finite()
            && self.high.is_finite()
            && self.high >= self.v
            && self.v >= self.low
        {
            return;
        }

        panic!(
            "illegal ErrorFloat: (value: {}, low: {}, high: {})",
            self.v, self.low, self.high
        );
    }
}

impl ops::Neg for ErrorFloat {
    type Output = ErrorFloat;

    fn neg(self) -> Self::Output {
        let result = ErrorFloat {
            v: -self.v,
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
            v: self.v + rhs.v,
            low: next_float_down(self.lower_bound() + rhs.lower_bound()),
            high: next_float_up(self.upper_bound() + rhs.upper_bound()),
        };
    }
}

impl ops::Sub<ErrorFloat> for ErrorFloat {
    type Output = ErrorFloat;
    fn sub(self, rhs: ErrorFloat) -> Self::Output {
        let result = ErrorFloat {
            v: self.v - rhs.v,
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
            v: self.v * rhs.v,
            low: product[0].min(product[1]).min(product[2].min(product[3])),
            high: product[0].max(product[1]).max(product[2].max(product[3])),
        };

        result.check();
        return result;
    }
}

impl ops::Mul<f32> for ErrorFloat {
    type Output = ErrorFloat;

    fn mul(self, rhs: f32) -> Self::Output {
        return self * ErrorFloat::without_error(rhs);
    }
}

impl ops::Mul<ErrorFloat> for f32 {
    type Output = ErrorFloat;

    fn mul(self, rhs: ErrorFloat) -> Self::Output {
        return rhs * self;
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
            v: self.v / rhs.v,
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

pub fn quadratic(a: f32, b: f32, c: f32, t0: &mut f32, t1: &mut f32) -> bool {
    let discrim = b as f64 * b as f64 - 4.0 * a as f64 * c as f64;

    if discrim < 0.0 {
        return false;
    }
    let root_discrm = discrim.sqrt();
    let q = if b < 0.0 {
        -0.5 * (b as f64 - root_discrm)
    } else {
        -0.5 * (b as f64 + root_discrm)
    };

    *t0 = q as f32 / a;
    *t1 = (c as f64 / q) as f32;

    if *t0 > *t1 {
        mem::swap(t0, t1);
    }

    return true;
}

pub fn error_float_quadratic(
    A: ErrorFloat,
    B: ErrorFloat,
    C: ErrorFloat,
    t0: &mut ErrorFloat,
    t1: &mut ErrorFloat,
) -> bool {
    let discrim = B.v as f64 * B.v as f64 - 4.0 * A.v as f64 * C.v as f64;

    if discrim < 0.0 {
        return false;
    }

    let root_discrim = discrim.sqrt();
    let float_root_discrim = ErrorFloat::with_error(
        root_discrim as f32,
        (root_discrim * MACHINE_EPSILON as f64) as f32,
    );

    let q = if B.v < 0.0 {
        -0.5 * (B - float_root_discrim)
    } else {
        -0.5 * (B + float_root_discrim)
    };

    *t0 = q / A;
    *t1 = C / q;

    if t0.v > t1.v {
        mem::swap(t0, t1);
    }

    return true;
}
