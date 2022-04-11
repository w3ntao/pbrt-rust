use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::utility::to_point;
use crate::fundamental::vector3::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Arc<dyn Material>,
    bounds: BoundingBox,
}

impl Sphere {
    pub fn new(_center: Point, _radius: f32) -> Self {
        let min = _center + Point::new(-_radius, -_radius, -_radius);
        let max = _center + Point::new(_radius, _radius, _radius);
        return Self {
            center: _center,
            radius: _radius,
            bounds: BoundingBox::build(&[min, max]),
            material: Arc::new(NullMaterial {}),
        };
    }
}

fn get_sphere_uv(p: Point) -> (f32, f32) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    let theta = (-p.y).acos();
    let phi = (-p.z / p.x).atan() + std::f32::consts::PI;

    return (phi / (2.0 * std::f32::consts::PI), theta / std::f32::consts::PI);
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return Intersection::failure();
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return Intersection::failure();
            }
        }
        let root = root;
        let normal = (ray.get_point(root) - self.center) / self.radius;
        let hit_point = ray.get_point(root);

        let mut intersection = if dot(ray.direction, normal) < 0.0 {
            Intersection::from_outside(root, hit_point,
                                       normal, self.material.clone())
        } else {
            Intersection::from_inside(root, hit_point,
                                      -normal, self.material.clone())
        };

        let (u, v) = get_sphere_uv(to_point((hit_point - self.center).normalize()));
        intersection.u = u;
        intersection.v = v;

        return intersection;
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }
}
