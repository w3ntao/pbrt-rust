use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct Interval {
    pub low: Float,
    pub high: Float,
}

impl Interval {
    pub fn from_value_and_error(v: Float, error: Float) -> Interval {
        if error == 0.0 {
            return Interval { low: v, high: v };
        }

        return Interval {
            low: sub_round_down(v, error),
            high: add_round_up(v, error),
        };
    }

    pub fn exactly(&self, v: Float) -> bool {
        return v == self.low && v == self.high;
    }

    pub fn midpoint(&self) -> Float {
        return (self.low + self.high) * 0.5;
    }

    pub fn width(&self) -> Float {
        return self.high - self.low;
    }

    pub fn contain_float_in_range(&self, v: Float) -> bool {
        return v >= self.low && v <= self.high;
    }

    pub fn sqr(&self) -> Interval {
        let alow = self.low.abs();
        let ahigh = self.high.abs();

        let (low, high) = if alow < ahigh {
            (alow, ahigh)
        } else {
            (ahigh, alow)
        };

        if self.contain_float_in_range(0.0) {
            return Interval {
                low: 0.0,
                high: mul_round_up(high, high),
            };
        }

        return Interval {
            low: mul_round_down(low, low),
            high: mul_round_up(high, high),
        };
    }

    pub fn sqrt(&self) -> Interval {
        return Interval {
            low: sqrt_round_down(self.low),
            high: sqrt_round_up(self.high),
        };
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ Interval [{}, {}] ]", self.low, self.high)
    }
}

impl From<Interval> for Float {
    fn from(value: Interval) -> Self {
        return value.midpoint();
    }
}

impl From<Float> for Interval {
    fn from(value: Float) -> Self {
        return Interval {
            low: value,
            high: value,
        };
    }
}

impl PartialEq<Interval> for Interval {
    fn eq(&self, other: &Interval) -> bool {
        return self.low == other.low && self.high == other.high;
    }
}

impl PartialEq<Float> for Interval {
    fn eq(&self, other: &Float) -> bool {
        return self.exactly(*other);
    }
}

impl Add<Interval> for Interval {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        return Interval {
            low: add_round_down(self.low, rhs.low),
            high: add_round_up(self.high, rhs.high),
        };
    }
}

impl Add<Float> for Interval {
    type Output = Interval;

    fn add(self, rhs: Float) -> Self::Output {
        return self + Interval::from(rhs);
    }
}

impl Sub<Interval> for Interval {
    type Output = Interval;

    fn sub(self, rhs: Interval) -> Self::Output {
        return Interval {
            low: sub_round_down(self.low, rhs.high),
            high: sub_round_up(self.high, rhs.low),
        };
    }
}

impl Mul<Float> for Interval {
    type Output = Interval;

    fn mul(self, f: Float) -> Self::Output {
        return if f > 0.0 {
            Interval {
                low: mul_round_down(f, self.low),
                high: mul_round_up(f, self.high),
            }
        } else {
            Interval {
                low: mul_round_down(f, self.high),
                high: mul_round_up(f, self.low),
            }
        };
    }
}

impl Mul<Interval> for Float {
    type Output = Interval;

    fn mul(self, rhs: Interval) -> Self::Output {
        return rhs * self;
    }
}

impl Mul<Interval> for Interval {
    type Output = Interval;

    fn mul(self, i: Interval) -> Self::Output {
        let lp = [
            mul_round_down(self.low, i.low),
            mul_round_down(self.high, i.low),
            mul_round_down(self.low, i.high),
            mul_round_down(self.high, i.high),
        ];

        let hp = [
            mul_round_up(self.low, i.low),
            mul_round_up(self.high, i.low),
            mul_round_up(self.low, i.high),
            mul_round_up(self.high, i.high),
        ];

        return Interval {
            low: lp.iter().fold(Float::INFINITY, |a, &b| a.min(b)),
            high: hp.iter().fold(-Float::INFINITY, |a, &b| a.max(b)),
        };
    }
}

impl Mul<Vector3<Interval>> for Interval {
    type Output = Vector3<Interval>;

    fn mul(self, rhs: Vector3<Interval>) -> Self::Output {
        return rhs * self;
    }
}

impl Div<Float> for Interval {
    type Output = Interval;

    fn div(self, f: Float) -> Self::Output {
        /*
           if (f == 0)
               return Interval(-Infinity, Infinity);

           if (f > 0)
               return Interval(DivRoundDown(i.LowerBound(), f), DivRoundUp(i.UpperBound(), f));
           else
               return Interval(DivRoundDown(i.UpperBound(), f), DivRoundUp(i.LowerBound(), f));

        */
        if f == 0.0 {
            return Interval {
                low: -Float::INFINITY,
                high: Float::INFINITY,
            };
        }

        return if f > 0.0 {
            Interval {
                low: div_round_down(self.low, f),
                high: div_round_up(self.high, f),
            }
        } else {
            Interval {
                low: div_round_down(self.high, f),
                high: div_round_up(self.low, f),
            }
        };
    }
}

impl Div<Interval> for Interval {
    type Output = Interval;

    fn div(self, i: Interval) -> Self::Output {
        if self.contain_float_in_range(0.0) {
            // The interval we're dividing by straddles zero, so just
            // return an interval of everything.
            return Interval {
                low: -Float::INFINITY,
                high: Float::INFINITY,
            };
        }

        let lowQuot = [
            div_round_down(self.low, i.low),
            div_round_down(self.high, i.low),
            div_round_down(self.low, i.high),
            div_round_down(self.high, i.high),
        ];

        let highQuot = [
            div_round_up(self.low, i.low),
            div_round_up(self.high, i.low),
            div_round_up(self.low, i.high),
            div_round_up(self.high, i.high),
        ];

        return Interval {
            low: lowQuot.iter().fold(Float::INFINITY, |a, &b| a.min(b)),
            high: highQuot.iter().fold(-Float::INFINITY, |a, &b| a.max(b)),
        };
    }
}
