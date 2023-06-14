use crate::pbrt::*;

pub struct SimpleIntegrator {
    camera: Arc<PerspectiveCamera>,
    sampler: Arc<SimpleSampler>,
}

impl SimpleIntegrator {
    pub fn new(camera: Arc<PerspectiveCamera>, sampler: Arc<SimpleSampler>) -> Self {
        return SimpleIntegrator { camera, sampler };
    }
}
