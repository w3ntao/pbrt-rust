use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::camera::Camera;
use crate::ray_tracing::ray::Ray;

pub struct DepthOfField {
    center: Point,
    forward: Vector3,

    horizontal: Vector3,
    vertical: Vector3,

    x_pixel_multiplier: f32,
    y_pixel_multiplier: f32,

    aperture: f32,
    focus_distance: f32,
}

impl DepthOfField {
    pub fn new(
        _center: Point,
        _forward: Vector3,
        _up: Vector3,
        _horizontal_opening_angle: f32,
        height_to_width_ratio: f32,
        _aperture: f32,
        _focus_distance: f32,
    ) -> Self {
        let _forward = _forward.normalize();
        let _up = _up.normalize();
        let _horizontal = cross(_forward, _up).normalize();
        let _x_pixel_multiplier = (_horizontal_opening_angle / 2.0).tan();

        return Self {
            center: _center,
            forward: _forward,
            horizontal: _horizontal,
            vertical: cross(_horizontal, _forward),

            x_pixel_multiplier: _x_pixel_multiplier,
            y_pixel_multiplier: _x_pixel_multiplier * height_to_width_ratio,

            aperture: _aperture,
            focus_distance: _focus_distance,
        };
    }
}

impl Camera for DepthOfField {
    fn get_primary_ray(&self, u: f32, v: f32) -> Ray {
        // u, v are both in [-1, 1]

        let x = u * self.x_pixel_multiplier;
        let y = v * self.y_pixel_multiplier;

        let direction = self.forward + x * self.horizontal + y * self.vertical;
        let target = self.center + direction.normalize() * self.focus_distance;

        let (rd_x, rd_y) = random_vector_in_disk();
        let rd_x = rd_x * self.aperture / 2.0;
        let rd_y = rd_y * self.aperture / 2.0;
        let origin = self.center + self.horizontal * rd_x + self.vertical * rd_y;

        return Ray::new(origin, (target - origin).normalize());
    }

    fn get_stratified_rays(
        &self,
        num_samples: u32,
        min_u: f32,
        max_u: f32,
        min_v: f32,
        max_v: f32,
    ) -> Vec<Ray> {
        let mut generator_u = thread_rng();
        let mut generator_v = thread_rng();

        let samples_per_dimension = (num_samples as f32).sqrt() as u32;

        let mut rays = vec![];

        let u_unit = (max_u - min_u) / (samples_per_dimension as f32);
        let v_unit = (max_v - min_v) / (samples_per_dimension as f32);

        for u_idx in 0..samples_per_dimension {
            let u_idx = u_idx as f32;
            let range_u = Uniform::new(min_u + u_unit * u_idx, min_u + u_unit * (u_idx + 1.0));

            for v_idx in 0..samples_per_dimension {
                let v_idx = v_idx as f32;
                let range_v = Uniform::new(min_v + v_unit * v_idx, min_v + v_unit * (v_idx + 1.0));

                let u = generator_u.sample(range_u);
                let v = generator_v.sample(range_v);

                let x = u * self.x_pixel_multiplier;
                let y = v * self.y_pixel_multiplier;
                let direction = self.forward + x * self.horizontal + y * self.vertical;

                let target = self.center + direction.normalize() * self.focus_distance;

                let (rd_x, rd_y) = random_vector_in_disk();
                let rd_x = rd_x * self.aperture / 2.0;
                let rd_y = rd_y * self.aperture / 2.0;
                let origin = self.center + self.horizontal * rd_x + self.vertical * rd_y;

                rays.push(Ray::new(origin, (target - origin).normalize()));
            }
        }

        return rays;
    }
}
