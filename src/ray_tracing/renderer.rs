use crate::Image;
//use crate::ray_tracing::cameras::perspective_camera::*;
use crate::ray_tracing::camera::Camera;
use crate::ray_tracing::integrators::ray_casting::*;

pub struct Renderer<'life> {
    camera: &'life (dyn Camera + 'life),
    integrator: RayCastingIntegrator,
}

impl<'life> Renderer<'life> {
    pub fn new(_camera: &'life (dyn Camera), _integrator: RayCastingIntegrator) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
        };
    }

    pub fn render<'a>(self, width: usize, height: usize) -> Image {
        let mut image = Image::new(width, height);
        for x in 0usize..image.width {
            let ndc_x = 2.0 * (x as f32) / (image.width as f32) - 1.0;
            for y in 0usize..image.height {
                let ndc_y = -2.0 * (y as f32) / (image.height as f32) + 1.0;
                let ray = self.camera.get_primary_ray(
                    ndc_x + 1.0 / (image.width as f32),
                    ndc_y - 1.0 / (image.height as f32));
                image.fill(self.integrator.get_radiance(&ray), y, x);
            }
        }
        return image;
    }
}
