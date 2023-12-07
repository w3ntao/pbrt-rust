use crate::pbrt::*;

pub struct SampledLight {
    pub light: Arc<dyn Light>,
    pub p: Float,
}

pub trait LightSampler {
    fn sample(&self, u: Float) -> Option<SampledLight>;
}
