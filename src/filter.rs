use crate::pbrt::*;

pub struct PixelSample {
    pub p: Point2f,
    pub weight: Float,
}

pub struct BoxFilter {
    pub radius: Float,
}

impl BoxFilter {
    pub fn new(_radius: Float) -> Self {
        return BoxFilter { radius: _radius };
    }

    pub fn evaluate(self, p: Point2f) -> Float {
        return if p.x.abs() <= self.radius && p.y.abs() <= self.radius {
            1.0
        } else {
            0.0
        };
    }
}
