use crate::core::pbrt::*;

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Mirror {
        Mirror {}
    }
}

impl Material for Mirror {
    fn scatter(
        &self,
        incoming_ray: Ray,
        surface_interaction: &SurfaceInteraction,
    ) -> (bool, Ray, Color) {
        let scattered_ray = Ray::new(
            surface_interaction.p,
            incoming_ray.d.reflect(surface_interaction.n),
            INTERSECT_EPSILON,
            f32::INFINITY,
        );

        return (true, scattered_ray, Color::new(1.0, 1.0, 1.0));
    }

    fn is_specular(&self) -> bool {
        return true;
    }
}
