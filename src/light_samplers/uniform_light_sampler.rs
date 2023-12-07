use crate::pbrt::*;

pub struct UniformLightSampler {
    lights: Vec<Arc<dyn Light>>,
}

impl LightSampler for UniformLightSampler {
    fn sample(&self, u: Float) -> Option<SampledLight> {
        if self.lights.is_empty() {
            return None;
        }

        let light_size = self.lights.len() as Float;

        return Some(SampledLight {
            light: self.lights[(u * light_size) as usize].clone(),
            p: 1.0 / light_size,
        });
    }
}
