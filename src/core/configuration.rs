use crate::core::pbrt::*;

pub struct Configuration {
    pub scene: Arc<Scene>,
    pub camera: Arc<dyn Camera>,
    pub integrator: Arc<dyn Integrator>,
    pub sampler: Arc<dyn Sampler>,
}

impl Configuration {
    pub fn new(
        _scene: Arc<Scene>,
        _camera: Arc<dyn Camera>,
        _integrator: Arc<dyn Integrator>,
        _sampler: Arc<dyn Sampler>,
    ) -> Self {
        return Self {
            scene: _scene,
            camera: _camera,
            integrator: _integrator,
            sampler: _sampler,
        };
    }
}
