use crate::core::interfaces::*;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(_albedo: Color, _fuzz: f32) -> Metal {
        Metal {
            albedo: _albedo,
            fuzz: _fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, incoming_ray: Ray, intersection: &Intersection) -> (bool, Ray, Color) {
        let reflected = incoming_ray.d.reflect(intersection.normal);

        let scattered_ray = Ray::new(
            intersection.hit_point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );

        return (
            scattered_ray.d.dot(intersection.normal) > 0.0,
            scattered_ray,
            self.albedo,
        );
        // for those light go beneath the surface, consider them not scattered
    }

    fn is_specular(&self) -> bool {
        return true;
    }
}
