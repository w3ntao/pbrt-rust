use crate::fundamental::rgb_color::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;

pub struct Lambertian {
    albedo: RGBColor,
}

impl Material for Lambertian {
    fn scatter(&self, attenuation: &mut RGBColor, scattered_ray: &mut Ray, incoming_ray: &Ray, intersect: &Intersection) -> bool {
        let scattered_direction = intersect.normal;
        //TODO: implement random_unit_vector()

        scattered_ray.origin = intersect.ray.get_point(intersect.distance);
        scattered_ray.direction = scattered_direction;
        *attenuation = self.albedo;
        return true;
    }
}
