use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::camera::Camera;
use crate::ray_tracing::ray::Ray;

pub struct Perspective {
    center: Point,
    forward: Vector3,
    horizontal: Vector3,

    image_plane_vertical: Vector3,
    x_pixel_multiplier: f32,
    y_pixel_multiplier: f32,
}

impl Perspective {
    pub fn new(_center: Point, _forward: Vector3, _up: Vector3,
               _vertical_opening_angle: f32, _horizontal_opening_angle: f32) -> Self {
        let _forward = _forward.normalize();
        let _up = _up.normalize();
        let _horizontal = cross(_forward, _up).normalize();

        return Self {
            center: _center,
            forward: _forward,
            horizontal: _horizontal,
            image_plane_vertical: cross(_horizontal, _forward),

            x_pixel_multiplier: (_horizontal_opening_angle / 2.0).tan(),
            y_pixel_multiplier: (_vertical_opening_angle / 2.0).tan(),
        };
    }
}

impl Camera for Perspective {
    fn get_primary_ray(&self, u: f32, v: f32) -> Ray {
        // u, v are both in [-1, 1]

        let x = u * self.x_pixel_multiplier;
        let y = v * self.y_pixel_multiplier;

        let direction = self.forward + x * self.horizontal + y * self.image_plane_vertical;

        return Ray::new(self.center, direction.normalize());
    }
}
