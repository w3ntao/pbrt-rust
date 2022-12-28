use crate::core::pbrt::*;

pub struct Perspective {
    center: Point,
    forward: Vector3,
    horizontal: Vector3,

    vertical: Vector3,
    x_pixel_multiplier: f32,
    y_pixel_multiplier: f32,
}

impl Perspective {
    pub fn new(
        _center: Point,
        _forward: Vector3,
        _up: Vector3,
        _horizontal_opening_angle: f32,
        height_to_width_ratio: f32,
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
        };
    }
}

impl Camera for Perspective {
    fn get_stratified_rays(
        &self,
        num_samples: u32,
        min_u: f32,
        max_u: f32,
        min_v: f32,
        max_v: f32,
    ) -> Vec<Ray> {
        if num_samples == 1 {
            let x = (max_u + min_u) / 2.0 * self.x_pixel_multiplier;
            let y = (max_v + min_v) / 2.0 * self.y_pixel_multiplier;
            let direction = self.forward + x * self.horizontal + y * self.vertical;

            return vec![Ray::new(self.center, direction.normalize(), f32::INFINITY)];
        }

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

                rays.push(Ray::new(self.center, direction.normalize(), f32::INFINITY));
            }
        }

        return rays;
    }

    fn get_ray(
        &self,
        min_u: f32,
        max_u: f32,
        min_v: f32,
        max_v: f32,
        sampler: &mut dyn Sampler,
    ) -> Ray {
        let (random_u, random_v) = sampler.get_2d_sample();
        let u = min_u + random_u * (max_u - min_u);
        let v = min_v + random_v * (max_v - min_v);

        let x = u * self.x_pixel_multiplier;
        let y = v * self.y_pixel_multiplier;
        let direction = self.forward + x * self.horizontal + y * self.vertical;

        return Ray::new(self.center, direction.normalize(), f32::INFINITY);
    }
}
