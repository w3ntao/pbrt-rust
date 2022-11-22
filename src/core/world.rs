use crate::core::pbrt::*;

pub struct World {
    lights: Vec<Arc<dyn Primitive>>,
    scene: BVH,
}

impl Default for World {
    fn default() -> Self {
        return Self {
            lights: vec![],
            scene: BVH::default(),
        };
    }
}

impl World {
    pub fn add(&mut self, object: Arc<dyn Primitive>) {
        self.scene.add(object);
    }

    pub fn add_light(&mut self, light: Arc<dyn Primitive>) {
        self.lights.push(light.clone());
        self.scene.add(light);
    }

    pub fn build_index(&mut self) {
        self.scene.build_index();
    }

    pub fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let mut ray = *ray;
        return self.scene.intersect(&mut ray, interaction);
    }

    pub fn sample_light(&self) -> (Point, Vector3, f32, Arc<dyn Material>) {
        let idx = thread_rng().gen_range(0..self.lights.len());

        let random_light = &(self.lights[idx]);
        let (point, normal) = random_light.sample();

        return (
            point,
            normal,
            random_light.get_area(),
            random_light.get_material(),
        );
    }
}
