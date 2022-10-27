use crate::core::pbrt::*;

pub struct Instance {
    pub primitive: Arc<dyn Primitive>,
    transform: Transform,
    material: Arc<dyn Material>,
}

impl Primitive for Instance {
    fn intersect(
        &self,
        ray: &Ray,
        t_min: f32,
        surface_interaction: &mut SurfaceInteraction,
    ) -> bool {
        let (inverted_ray, inverted_distance) = (self.transform)(ray);

        if !self.primitive.intersect(
            &inverted_ray,
            t_min / inverted_distance,
            surface_interaction,
        ) {
            return false;
        }

        surface_interaction.n = (self.transform)(surface_interaction.n);
        surface_interaction.t = surface_interaction.t / inverted_distance;
        surface_interaction.p = ray(surface_interaction.t);

        if !self.material.is_null() {
            surface_interaction.material = self.material.clone();
        }

        return true;
    }

    fn get_bounds(&self) -> Bounds {
        let bounds = self.primitive.get_bounds();
        let min = bounds.p_min;
        let max = bounds.p_max;

        let mut points = [
            min,
            max,
            Point::new(max.x, min.y, min.z),
            Point::new(max.x, min.y, max.z),
            Point::new(max.x, max.y, min.z),
            Point::new(min.x, max.y, min.z),
            Point::new(min.x, min.y, max.z),
            Point::new(min.x, max.y, max.z),
        ];

        for idx in 0..points.len() {
            points[idx] = (self.transform)(points[idx])
        }

        return Bounds::build(&points);
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn sample(&self) -> (Point, Vector3) {
        panic!("sample() is not implemented for Instance");
    }

    fn get_area(&self) -> f32 {
        panic!("get_area() is not implemented for Instance");
    }
}

impl Instance {
    pub fn new(_primitive: Arc<dyn Primitive>) -> Instance {
        Instance {
            primitive: _primitive,
            transform: Transform::identity(),
            material: Arc::new(NullMaterial {}),
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.transform.reset();
    }

    pub fn translate(&mut self, t: Vector3) {
        self.transform.translate(t);
    }

    pub fn scale_by_scalar(&mut self, scalar: f32) {
        self.transform.scale_by_scalar(scalar);
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        self.transform.rotate(axis, angle);
    }
}
