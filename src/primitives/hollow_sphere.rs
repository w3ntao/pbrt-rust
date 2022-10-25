use crate::core::pbrt::*;

pub struct HollowSphere {
    pub external_sphere: Sphere,
    pub internal_sphere: Sphere,
}

impl HollowSphere {
    pub fn new(_center: Point, _radius: f32, thickness: f32) -> Self {
        return Self {
            external_sphere: Sphere::new(_center, _radius),
            internal_sphere: Sphere::new(_center, _radius - thickness),
        };
    }
}

impl Primitive for HollowSphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> SurfaceInteraction {
        let external_surface_interaction = self.external_sphere.intersect(ray, t_min, t_max);
        if !external_surface_interaction.intersected()
            || external_surface_interaction.entering_material
        {
            return external_surface_interaction;
        }

        let mut internal_surface_interaction =
            self.internal_sphere
                .intersect(ray, INTERSECT_OFFSET, external_surface_interaction.t);
        if !internal_surface_interaction.intersected() {
            return external_surface_interaction;
        }

        internal_surface_interaction.entering_material =
            !internal_surface_interaction.entering_material;

        return internal_surface_interaction;
    }

    fn get_bounds(&self) -> Bounds {
        return self.external_sphere.get_bounds();
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.external_sphere.set_material(material.clone());
        self.internal_sphere.set_material(material);
    }
}
