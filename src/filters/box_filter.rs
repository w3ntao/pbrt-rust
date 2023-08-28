use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct BoxFilter {
    pub radius: Vector2f,
}

impl BoxFilter {
    pub fn new(_radius: Float) -> Self {
        return BoxFilter {
            radius: Vector2f::new(_radius, _radius),
        };
    }
}

impl Filter for BoxFilter {
    fn sample(&self, u: Point2f) -> FilterSample {
        let p = Point2f::new(
            lerp(u.x, -self.radius.x, self.radius.x),
            lerp(u.y, -self.radius.y, self.radius.y),
        );

        return FilterSample {
            p,
            weight: 1.0 as Float,
        };
    }

    fn evaluate(&self, p: Point2f) -> Float {
        return if p.x.abs() <= self.radius.x && p.y.abs() <= self.radius.y {
            1.0
        } else {
            0.0
        };
    }
}
