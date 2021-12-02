use crate::fundamental::point::*;
use crate::fundamental::vector::*;
use crate::ray::*;

pub struct PerspectiveCamera {
    center: Point,
    forward: Vector,
    horizontal: Vector,

    image_plane_vertical: Vector,
    x_pixel_multiplier: f32,
    y_pixel_multiplier: f32,
}

impl PerspectiveCamera {
    pub fn new(_center: Point, _forward: Vector, _up: Vector,
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

    pub fn get_primary_ray(&self, x: f32, y: f32) -> Ray {
        let x = x * self.x_pixel_multiplier;
        let y = y * self.y_pixel_multiplier;

        let direction = self.forward + x * self.horizontal + y * self.image_plane_vertical;

        return Ray::new(self.center, direction.normalize());
    }
}
