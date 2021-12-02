use crate::group::*;
use crate::perspective_camera::*;
use crate::ray_casting_integrator::*;

pub struct Renderer {
    camera: *const PerspectiveCamera,
    integrator: *const RayCastingIntegrator,
}

impl Renderer {
    pub fn new(_camera: &PerspectiveCamera, _integrator: &RayCastingIntegrator) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
        };
    }
}
