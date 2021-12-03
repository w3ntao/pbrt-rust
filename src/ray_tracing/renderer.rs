use crate::Image;
use crate::ray_tracing::camera_trait::Camera;
use crate::ray_tracing::integrator_trait::Integrator;

pub struct Renderer<'a> {
    camera: &'a dyn Camera,
    integrator: &'a dyn Integrator,
}

impl<'a> Renderer<'a> {
    pub fn new(_camera: &'a dyn Camera, _integrator: &'a dyn Integrator) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
        };
    }

    pub fn render(self, width: usize, height: usize) -> Image {
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
