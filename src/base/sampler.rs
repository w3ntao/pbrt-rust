use crate::pbrt::*;

pub struct SimpleSampler {
    rng: StdRng,
}

impl SimpleSampler {
    pub fn new() -> Self {
        return SimpleSampler {
            rng: StdRng::from_entropy(),
        };
    }

    pub fn get_1d(&mut self) -> Float {
        return self.rng.gen::<f32>();
    }

    pub fn get_2d(&mut self) -> Point2f {
        return Point2f::new(self.rng.gen::<f32>(), self.rng.gen::<f32>());
    }

    pub fn get_pixel_2d(&mut self) -> Point2f {
        return self.get_2d();
    }
}
