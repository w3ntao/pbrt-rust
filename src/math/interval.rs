use crate::pbrt::*;

pub struct Interval {
    pub low: Float,
    pub high: Float,
}

impl Interval {
    pub fn new(v: Float) -> Interval {
        return Interval { low: v, high: v };
    }

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
