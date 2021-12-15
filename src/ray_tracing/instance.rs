use std::sync::Arc;
use crate::fundamental::vector::Vector;
use crate::fundamental::float4::Float4;
use crate::fundamental::matrix::*;
use crate::fundamental::point::Point;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::Ray;

pub struct Instance {
    pub primitive: Arc<dyn Primitive>,
    transform: Matrix,
}

impl Primitive for Instance {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let inverted_transform = self.transform.invert();
        let mut inverted_ray_direction = inverted_transform.clone() * ray.direction;
        let inverted_length = inverted_ray_direction.length();
        inverted_ray_direction = inverted_ray_direction / inverted_length;

        let intersect = self.primitive.intersect(
            &Ray::new(inverted_transform.clone() * ray.origin, inverted_ray_direction),
            previous_distance / inverted_length);
        if !intersect.intersected() {
            // failure
            return Intersection::failure();
        }

        return Intersection::new(intersect.distance / inverted_length, &ray,
                                 (inverted_transform.transpose() * intersect.normal).normalize());
    }

    fn get_bounds(&self) -> BoundingBox {
        let bounds = self.primitive.get_bounds();
        let min = bounds.min;
        let max = bounds.max;

        let mut points = [
            min, max,
            Point::new(max.x, min.y, min.z),
            Point::new(min.x, max.y, min.z),
            Point::new(min.x, min.y, max.z),
            Point::new(min.x, max.y, max.z),
            Point::new(max.x, min.y, max.z),
            Point::new(max.x, max.y, min.z),
        ];

        for idx in 0..points.len() {
            points[idx] = self.transform.clone() * points[idx];
        }

        return BoundingBox::build(&points);
    }
}

impl Instance {
    pub fn new(_primitive: Arc<dyn Primitive>) -> Instance {
        Instance {
            primitive: _primitive,
            transform: Matrix::identity(),
        }
    }

    pub fn reset(&mut self) {
        self.transform = Matrix::identity();
    }

    pub fn translate(&mut self, t: &Vector) {
        for idx in 0..3 {
            self.transform[idx][3] += t[idx];
        }
    }

    pub fn scale_by_scalar(&mut self, scalar: f32) {
        for idx in 0..3 {
            self.transform[idx][idx] *= scalar;
        }
    }

    pub fn scale_by_vector(&mut self, scalar: Vector) {
        for idx in 0..3 {
            self.transform[idx][idx] *= scalar[idx];
        }
    }

    pub fn rotate(&mut self, axis: &Vector, angle: f32) {
        let cosine = f32::cos(angle);
        let sine = f32::sin(angle);

        let normalized_axis = axis.normalize();
        let x = normalized_axis.x;
        let y = normalized_axis.y;
        let z = normalized_axis.z;

        let rotate_matrix = Matrix::new(
            &Float4::new(
                x * x * (1.0 - cosine) + cosine,
                x * y * (1.0 - cosine) - z * sine,
                x * z * (1.0 - cosine) + y * sine,
                0.0),
            &Float4::new(
                x * y * (1.0 - cosine) + z * sine,
                cosine + y * y * (1.0 - cosine),
                y * z * (1.0 - cosine) - x * sine,
                0.0),
            &Float4::new(
                x * z * (1.0 - cosine) - y * sine,
                y * z * (1.0 - cosine) + x * sine,
                cosine + z * z * (1.0 - cosine),
                0.0),
            &Float4::new(0.0, 0.0, 0.0, 1.0));

        self.transform = product(&rotate_matrix, &self.transform);
    }
}
