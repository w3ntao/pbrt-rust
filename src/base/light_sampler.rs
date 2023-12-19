use crate::pbrt::*;

pub struct SampledLight {
    pub light: Arc<dyn Light>,
    pub p: f64,
}

pub trait LightSampler {
    fn sample(&self, u: f64) -> Option<SampledLight>;
}
