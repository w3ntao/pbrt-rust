use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Perspective {
    center: Point,
    forward: Vector3,
    horizontal: Vector3,

    vertical: Vector3,
    x_pixel_multiplier: f32,

    lens_radius: f32,
    focus_distance: f32,
}

impl Perspective {
    pub fn without_lens(
        _center: Point,
        _forward: Vector3,
        _up: Vector3,
        _horizontal_opening_angle: f32,
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

            lens_radius: 0.0,
            focus_distance: f32::NAN,
        };
    }

    pub fn with_lens(
        _center: Point,
        _forward: Vector3,
        _up: Vector3,
        _horizontal_opening_angle: f32,
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
        ndc_x: f32,
        ndc_y: f32,
        width: usize,
        height: usize,
        sampler: &mut dyn Sampler,
    ) -> Ray {
        let min_u = ndc_x;
        let max_u = ndc_x + 2.0 / (width as f32);
        let min_v = ndc_y - 2.0 / (height as f32);
        let max_v = ndc_y;

        let y_pixel_multiplier = self.x_pixel_multiplier * (height as f32 / width as f32);

        let (random_u, random_v) = sampler.get_2d_sample();
        let u = min_u + random_u * (max_u - min_u);
        let v = min_v + random_v * (max_v - min_v);

        let offset_x = u * self.x_pixel_multiplier;
        let offset_y = v * y_pixel_multiplier;
        let direction = self.forward + offset_x * self.horizontal + offset_y * self.vertical;

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

    fn reset_lens_and_focus_distance(
        &self,
        lens_radius: f32,
        focus_distance: f32,
    ) -> Arc<Perspective> {
        let mut new_camera = *self;
        new_camera.lens_radius = lens_radius;
        new_camera.focus_distance = focus_distance;

        return Arc::new(new_camera);
    }
}
