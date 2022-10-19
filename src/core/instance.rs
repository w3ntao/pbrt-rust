use crate::core::interfaces::*;
use crate::core::matrix::*;
use std::sync::Arc;

pub struct Instance {
    pub primitive: Arc<dyn Primitive>,
    transform: Matrix,
    material: Arc<dyn Material>,
}

impl Primitive for Instance {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let inverted_transform = self.transform.invert();
        let mut inverted_ray_direction = inverted_transform.clone() * ray.d;
        let inverted_length = inverted_ray_direction.length();
        inverted_ray_direction = inverted_ray_direction / inverted_length;

        let mut intersection = self.primitive.intersect(
            &Ray::new(inverted_transform.clone() * ray.o, inverted_ray_direction),
            t_min / inverted_length,
            t_max / inverted_length,
        );
        if !intersection.intersected() {
            return intersection;
        }

        intersection.normal = (inverted_transform.transpose() * intersection.normal).normalize();
        intersection.distance = intersection.distance / inverted_length;
        intersection.hit_point = ray.get_point(intersection.distance);

        if !self.material.is_null() {
            intersection.material = self.material.clone();
        }

        return intersection;
    }

    fn get_bounds(&self) -> Bounds {
        let bounds = self.primitive.get_bounds();
        let min = bounds.min;
        let max = bounds.max;

        let mut points = [
            min,
            max,
            Point::new(max.x, min.y, min.z),
            Point::new(max.x, min.y, max.z),
            Point::new(max.x, max.y, min.z),
            Point::new(min.x, max.y, min.z),
            Point::new(min.x, min.y, max.z),
            Point::new(min.x, max.y, max.z),
        ];

        for idx in 0..points.len() {
            points[idx] = self.transform.clone() * points[idx];
        }

        return Bounds::build(&points);
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn sample(&self) -> (Point, Vector3) {
        panic!("sample() is not implemented for Instance");
    }

    fn get_id(&self) -> u128 {
        panic!("get_id() is not implemented for Instance");
    }

    fn get_area(&self) -> f32 {
        panic!("get_area() is not implemented for Instance");
    }
}

impl Instance {
    pub fn new(_primitive: Arc<dyn Primitive>) -> Instance {
        Instance {
            primitive: _primitive,
            transform: Matrix::identity(),
            material: Arc::new(NullMaterial {}),
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.transform = Matrix::identity();
    }

    pub fn translate(&mut self, t: Vector3) {
        for idx in 0..3 {
            self.transform[idx][3] += t[idx];
        }
    }

    pub fn scale_by_scalar(&mut self, scalar: f32) {
        for row in 0..3 {
            for col in 0..3 {
                self.transform[row][col] *= scalar;
            }
        }
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        let cosine = f32::cos(angle);
        let sine = f32::sin(angle);

        let normalized_axis = axis.normalize();
        let x = normalized_axis.x;
        let y = normalized_axis.y;
        let z = normalized_axis.z;

        let rotate_matrix = Matrix::new(
            Vector4::new(
                x * x * (1.0 - cosine) + cosine,
                x * y * (1.0 - cosine) - z * sine,
                x * z * (1.0 - cosine) + y * sine,
                0.0,
            ),
            Vector4::new(
                x * y * (1.0 - cosine) + z * sine,
                cosine + y * y * (1.0 - cosine),
                y * z * (1.0 - cosine) - x * sine,
                0.0,
            ),
            Vector4::new(
                x * z * (1.0 - cosine) - y * sine,
                y * z * (1.0 - cosine) + x * sine,
                cosine + z * z * (1.0 - cosine),
                0.0,
            ),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        );

        self.transform = rotate_matrix * self.transform.clone();
    }
}
