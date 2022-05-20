use crate::fundamental::color::*;
use crate::fundamental::point::Point;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Mirror {
        Mirror {}
    }
}

impl Material for Mirror {
    fn scatter(&self, incoming_ray: &Ray, intersection: &Intersection) -> (Ray, Color) {
        let scattered_ray = Ray::new(intersection.hit_point, incoming_ray.direction.reflect(intersection.normal));

        return (scattered_ray, Color::new(1.0, 1.0, 1.0));
        // TODO: I am implementing perfect glass for the time being
        // TODO: that reflects everything
    }

    fn emit(&self, _: f32, _: f32, _: Point) -> Color {
        return Color::black();
    }
}
