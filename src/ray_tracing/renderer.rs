use std::rc::Rc;
use std::time::Instant;

use crate::fundamental::image::Image;
use crate::ray_tracing::camera::Camera;
use crate::ray_tracing::integrator::Integrator;

pub struct Renderer {
    camera: Rc<dyn Camera>,
    integrator: Rc<dyn Integrator>,
}

impl Renderer {
    pub fn new(_camera: Rc<dyn Camera>, _integrator: Rc<dyn Integrator>) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
        };
    }

    pub fn render(self, width: usize, height: usize) -> Image {
        let start = Instant::now();

        let mut image = Image::new(width, height);
        for x in 0usize..image.width {
            let ndc_x = 2.0 * (x as f32) / (image.width as f32) - 1.0;
            for y in 0usize..image.height {
                let ndc_y = -2.0 * (y as f32) / (image.height as f32) + 1.0;
                let ray = self.camera.get_primary_ray(
                    ndc_x + 1.0 / (image.width as f32),
                    ndc_y - 1.0 / (image.height as f32));
                image.fill(self.integrator.get_radiance(Rc::new(ray)), y, x);
            }
        }
        println!("Rendering took {:.2}[s]", start.elapsed().as_secs_f32());
        return image;
    }
}
