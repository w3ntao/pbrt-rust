use crate::core::pbrt::*;

pub struct Scene {
    lights: Vec<Arc<dyn Primitive>>,
    objects: BVH,
}

impl Default for Scene {
    fn default() -> Self {
        return Self {
            lights: vec![],
            objects: BVH::default(),
        };
    }
}

impl Scene {
    pub fn add(&mut self, object: Arc<dyn Primitive>) {
        self.objects.add(object);
    }

    pub fn add_light(&mut self, light: Arc<dyn Primitive>) {
        self.lights.push(light.clone());
        self.objects.add(light);
    }

    pub fn build_index(&mut self) {
        self.objects.build_index();
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let mut mut_ray = *ray;
        return self.objects.intersect(&mut mut_ray);
    }

    pub fn sample_light(
        &self,
        sampler: &mut dyn Sampler,
    ) -> (Point, Vector3, f32, Arc<dyn Material>) {
        let light_idx = sampler.get_1d_sample() * ((self.lights.len() as f32) - f32::EPSILON);
        let random_light = &(self.lights[light_idx as usize]);
        let (point, normal) = random_light.sample(sampler);

        return (
            point,
            normal,
            random_light.get_area(),
            random_light.get_material(),
        );
    }
}
