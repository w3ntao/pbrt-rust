use crate::Image;
use crate::perspective_camera::*;
use crate::ray_casting_integrator::*;

pub struct Renderer {
    camera: PerspectiveCamera,
    integrator: RayCastingIntegrator,
}

impl Renderer {
    pub fn new(_camera: PerspectiveCamera, _integrator: RayCastingIntegrator) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
        };
    }

    pub fn render(self, width: usize, height: usize) -> Image {
        let mut image = Image::new(width, height);
        for x in 0usize..image.width {
            let ndcX = 2.0 * (x as f32) / (image.width as f32) - 1.0;
            for y in 0usize..image.height {
                let ndcY = -2.0 * (y as f32) / (image.height as f32) + 1.0;
                let ray = self.camera.getPrimaryRay(
                    ndcX + 1.0 / (image.width as f32),
                    ndcY - 1.0 / (image.height as f32));
                image.fill(self.integrator.get_radiance(&ray), y, x);
            }
        }
        return image;
    }
}
