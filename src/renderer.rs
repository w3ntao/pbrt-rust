use std::ptr::null;
use crate::vector::*;
use crate::group::*;
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

    pub fn render(self, image_width: usize, image_height: usize) -> Vec<Vec<Vector>> {
        let factor = 256 as f32 - 0.001;

        let mut pixels = vec![vec![Color::zero(); image_width]; image_height];
        for x in 0usize..image_width {
            let ndcX = 2.0 * (x as f32) / (image_width as f32) - 1.0;
            for y in 0usize..image_height {
                let ndcY = -2.0 * (y as f32) / (image_height as f32) + 1.0;
                let ray = self.camera.getPrimaryRay(
                    ndcX + 1.0 / (image_width as f32),
                    ndcY - 1.0 / (image_height as f32));
                pixels[y][x] = self.integrator.get_radiance(&ray) * factor;
            }
        }

        return pixels;
    }

    pub fn dummy_render(self, image_width: usize, image_height: usize) -> Vec<Vec<Vector>> {
        let mut pixels = vec![vec![Color::zero(); image_width]; image_height];
        let factor = 256 as f32 - 0.001;
        for h in (0usize..image_height).rev() {
            for w in 0usize..image_width {
                let red = w as f32 / (image_width - 1) as f32;
                let green = h as f32 / (image_height - 1) as f32;
                let blue = 0.25;

                pixels[h][w] = Vector::new(red * factor,
                                           green * factor,
                                           blue * factor);
            }
        }
        return pixels;
    }
}
