use crate::vector::*;
use crate::ray::*;

pub struct PerspectiveCamera {
    center: Vector,
    forward: Vector,
    up: Vector,
    horizontal: Vector,

    imgPlaneVertical: Vector,
    xPixelMultiplier: f32,
    yPixelMultiplier: f32,
}

impl PerspectiveCamera {
    pub fn new(_center: Vector, _forward: Vector, _up: Vector,
               _verticalOpeningAngle: f32, _horizontalOpeningAngle: f32) -> Self {
        let _forward = _forward.normalize();
        let _up = _up.normalize();
        let _horizontal = cross(_forward, _up).normalize();

        return Self {
            center: _center,
            forward: _forward,
            up: _up,
            horizontal: _horizontal,
            imgPlaneVertical: cross(_horizontal, _forward),

            xPixelMultiplier: (_horizontalOpeningAngle / 2.0).tan(),
            yPixelMultiplier: (_verticalOpeningAngle / 2.0).tan(),
        };
    }

    pub fn getPrimaryRay(&self, x: f32, y: f32) -> Ray {
        let x = x * self.xPixelMultiplier;
        let y = y * self.yPixelMultiplier;

        let rayDirection = self.forward + x * self.horizontal + y * self.imgPlaneVertical;

        return Ray::new(self.center, rayDirection.normalize());
    }
}
