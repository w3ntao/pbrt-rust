use crate::core::pbrt::*;

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
    fn scatter(
        &self,
        incoming_ray: Ray,
        surface_interaction: &SurfaceInteraction,
    ) -> (bool, Ray, Color) {
        let reflected = incoming_ray.d.reflect(surface_interaction.n);

        let scattered_ray = Ray::new(
            surface_interaction.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            INTERSECT_EPSILON,
            f32::INFINITY,
        );

        return (
            surface_interaction.n.dot(scattered_ray.d) > 0.0,
            scattered_ray,
            self.albedo,
        );
        // for those light go beneath the surface, consider them not scattered
    }

    fn is_specular(&self) -> bool {
        return true;
    }
}
