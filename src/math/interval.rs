use crate::pbrt::*;

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

    pub fn midpoint(&self) -> Float {
        return (self.low + self.high) * 0.5;
    }

    pub fn width(&self) -> Float {
        return self.high - self.low;
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
