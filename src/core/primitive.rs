use crate::core::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, surface_interaction: &mut SurfaceInteraction) -> bool;

    fn set_material(&mut self, material: Arc<dyn Material>);

    fn get_bounds(&self) -> AABBbounds;

    fn get_area(&self) -> f32;

    fn sample(&self) -> (Point, Vector3);
}

pub trait Aggregate {
    fn add(&mut self, p: Arc<dyn Primitive>);
}

pub struct GeometricPrimitive {
    shape: Arc<dyn Shape>,
    material: Option<Arc<dyn Material>>,
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, ray: &Ray, surface_interaction: &mut SurfaceInteraction) -> bool {
        if !self.shape.intersect(&ray, surface_interaction) {
            return false;
        }

        match &self.material {
            Some(material) => {
                surface_interaction.material = Some(material.clone());
            }
            _ => {}
        }

        return true;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = Some(material);
    }

    fn get_bounds(&self) -> AABBbounds {
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
    pub fn new(_shape: Arc<dyn Shape>, _material: Arc<dyn Material>) -> GeometricPrimitive {
        GeometricPrimitive {
            shape: _shape,
            material: Some(_material),
        }
    }
}

pub struct TransformedPrimitive {
    primitive: Arc<dyn Primitive>,
    transform: Transform,
    material: Option<Arc<dyn Material>>,
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

        match &self.material {
            Some(material) => {
                surface_interaction.material = Some(material.clone());
            }
            _ => {}
        }

        return true;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = Some(material);
    }

    fn get_bounds(&self) -> AABBbounds {
        // a smarter way to transform bounds:
        // takes roughly 2 transforms instead of 8
        // https://stackoverflow.com/a/58630206

        let mut transformed_bounds = AABBbounds::empty();
        for idx in 0..3 {
            transformed_bounds.min[idx] = self.transform[idx][3];
        }
        transformed_bounds.max = transformed_bounds.min;

        let bounds = self.primitive.get_bounds();

        for i in 0..3 {
            for k in 0..3 {
                let a = self.transform[i][k] * bounds.min[k];
                let b = self.transform[i][k] * bounds.max[k];

                transformed_bounds.min[i] += if a < b { a } else { b };
                transformed_bounds.max[i] += if a < b { b } else { a };
            }
        }

        return transformed_bounds;
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
            material: None,
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
