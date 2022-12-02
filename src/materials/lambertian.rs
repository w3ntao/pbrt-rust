use crate::core::pbrt::*;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: texture }
    }
}

fn random_cosine_direction() -> Vector3 {
    let sin2_theta = random_f32(0.0, 1.0);
    let cos2_theta = 1.0 - sin2_theta;

    let phi = random_f32(0.0, 2.0 * PI);
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();

    let sin_theta = sin2_theta.sqrt();

    return Vector3::new(sin_phi * sin_theta, cos_phi * sin_theta, cos2_theta.sqrt());
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _incoming_ray: Ray,
        surface_interaction: &SurfaceInteraction,
        scattered_direction: &mut Vector3,
        attenuation: &mut Color,
    ) -> bool {
        let uvw = OrthonormalBasis::build_from_w(Vector3::from(surface_interaction.n));
        let random_direction = uvw.local(random_cosine_direction());

        *scattered_direction = random_direction.normalize();
        *attenuation = self.albedo.get_color(
            surface_interaction.u,
            surface_interaction.v,
            surface_interaction.p,
        );

        return true;
    }

    fn scattering_pdf(&self, _: Vector3, normal: Normal, scattered_direction: Vector3) -> f32 {
        let val_cosine = normal.cosine(scattered_direction);

        return if val_cosine <= 0.0 {
            0.0
        } else {
            val_cosine / PI
        };
    }
}
