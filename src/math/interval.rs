use crate::pbrt::*;

pub struct Interval {
    pub low: Float,
    pub high: Float,
}

impl Interval {
    pub fn new(v: Float) -> Interval {
        return Interval { low: v, high: v };
    }

    pub fn midpoint(&self) -> Float {
        return (self.low + self.high) * 0.5;
    }
}

impl From<Interval> for Float {
    fn from(value: Interval) -> Self {
        return value.midpoint();
    }
}
