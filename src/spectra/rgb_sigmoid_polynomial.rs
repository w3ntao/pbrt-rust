use crate::pbrt::*;

pub struct RGBSigmoidPolynomial {
    c0: f64,
    c1: f64,
    c2: f64,
}

impl RGBSigmoidPolynomial {
    pub fn new(c0: f64, c1: f64, c2: f64) -> Self {
        return Self { c0, c1, c2 };
    }

    pub fn eval(&self, lambda: f64) -> f64 {
        return sigmoid(evaluate_polynomial(lambda, &[self.c2, self.c1, self.c0]));
    }
}

impl Display for RGBSigmoidPolynomial {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "[ RGBSigmoidPolynomial c0: {}, c1: {}, c2: {} ]",
            self.c0, self.c1, self.c2
        )
    }
}
