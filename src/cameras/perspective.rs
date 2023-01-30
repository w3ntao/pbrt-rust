use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Perspective {
    center: Point,
    forward: Vector3,
    horizontal: Vector3,

    vertical: Vector3,
    x_pixel_multiplier: f32,
    y_pixel_multiplier: f32,

    lens_radius: f32,
    focus_distance: f32,
}

impl Perspective {
    pub fn without_lens(
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

            lens_radius: 0.0,
            focus_distance: f32::NAN,
        };
    }

    pub fn with_lens(
        _center: Point,
        _forward: Vector3,
        _up: Vector3,
        _horizontal_opening_angle: f32,
        height_to_width_ratio: f32,
        _lens_radius: f32,
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

            lens_radius: _lens_radius,
            focus_distance: _focus_distance,
        };
    }
}

fn random_vector_in_disk(sample: Sample2D) -> (f32, f32) {
    let (random_u, random_v) = sample;
    let r = random_u.sqrt();
    let theta = random_v * 2.0 * PI;

    return (r * theta.sin(), r * theta.cos());
}

impl Camera for Perspective {
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

        if self.lens_radius <= 0.0 {
            return Ray::new(self.center, direction.normalize(), f32::INFINITY);
        }

        let target = self.center + direction.normalize() * self.focus_distance;

        let (rd_x, rd_y) = random_vector_in_disk(sampler.get_2d_sample());
        let origin = self.center
            + rd_x * self.lens_radius * self.horizontal
            + rd_y * self.lens_radius * self.vertical;

        return Ray::new(origin, (target - origin).normalize(), f32::INFINITY);
    }

    fn remove_lens(&self) -> Arc<Perspective> {
        let mut new_camera = *self;
        new_camera.lens_radius = 0.0;
        new_camera.focus_distance = f32::NAN;

        return Arc::new(new_camera);
    }
}
