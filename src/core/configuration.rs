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

    pub fn update_camera(&self, _camera: Arc<dyn Camera>) -> Arc<Configuration> {
        return Arc::new(Configuration {
            scene: self.scene.clone(),
            camera: _camera,
            integrator: self.integrator.clone(),
            sampler: self.sampler.clone(),
        });
    }

    pub fn update_integrator(&self, _integrator: Arc<dyn Integrator>) -> Arc<Configuration> {
        return Arc::new(Configuration {
            scene: self.scene.clone(),
            camera: self.camera.clone(),
            integrator: _integrator,
            sampler: self.sampler.clone(),
        });
    }
}
