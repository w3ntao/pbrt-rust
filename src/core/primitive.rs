use crate::core::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, surface_interaction: &mut SurfaceInteraction) -> bool;

    fn set_material(&mut self, material: Arc<dyn Material>);

    fn get_bounds(&self) -> Bounds;

    fn get_area(&self) -> f32;

    fn sample(&self) -> (Point, Vector3);
}

pub trait Aggregate {
    fn add(&mut self, p: Arc<dyn Primitive>);
}

pub struct GeometricPrimitive {
    shape: Arc<dyn Shape>,
    material: Arc<dyn Material>,
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, ray: &Ray, surface_interaction: &mut SurfaceInteraction) -> bool {
        return self.shape.intersect(ray, surface_interaction);
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn get_bounds(&self) -> Bounds {
        return self.shape.get_bounds();
    }

    fn get_area(&self) -> f32 {
        return self.shape.get_area();
    }

    fn sample(&self) -> (Point, Vector3) {
        return self.shape.sample();
    }
}

impl GeometricPrimitive {
    pub fn new(_shape: Arc<dyn Shape>) -> GeometricPrimitive {
        GeometricPrimitive {
            shape: _shape,
            material: Arc::new(NullMaterial {}),
        }
    }
}

pub struct TransformedPrimitive {
    primitive: Arc<dyn Primitive>,
    transform: Transform,
    material: Arc<dyn Material>,
}

impl Primitive for TransformedPrimitive {
    fn intersect(&self, ray: &Ray, surface_interaction: &mut SurfaceInteraction) -> bool {
        let (inverted_ray, inverted_distance) = (self.transform)(ray);

        if !self.primitive.intersect(&inverted_ray, surface_interaction) {
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

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
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

    fn get_area(&self) -> f32 {
        let area = self.primitive.get_area();
        return if self.transform.is_identity() {
            area
        } else {
            area * self.transform.determinant()
        };
    }

    fn sample(&self) -> (Point, Vector3) {
        let (p, v) = self.primitive.sample();
        return if self.transform.is_identity() {
            (p, v)
        } else {
            ((self.transform)(p), (self.transform)(v))
        };
    }
}

impl TransformedPrimitive {
    pub fn new(_primitive: Arc<dyn Primitive>) -> TransformedPrimitive {
        TransformedPrimitive {
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
