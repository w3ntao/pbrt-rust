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
        self.scene.add(object.clone());
    }

    pub fn add_light(&mut self, light: Arc<dyn Primitive>) {
        self.lights.push(light.clone());
        self.scene.add(light.clone());
    }

    pub fn build_index(&mut self) {
        self.scene.build_index();
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, interaction: &mut SurfaceInteraction) -> bool {
        return self.scene.intersect(ray, t_min, interaction);
    }

    pub fn sample_light(&self) -> (u128, Point, Vector3, f32) {
        let idx = thread_rng().gen_range(0..self.lights.len());
        let (point, normal) = self.lights[idx].sample();

        return (
            self.lights[idx].get_id(),
            point,
            normal,
            self.lights[idx].get_area(),
        );
    }
}
