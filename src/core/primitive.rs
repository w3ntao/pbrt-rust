use crate::core::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &mut Ray, surface_interaction: &mut SurfaceInteraction) -> bool;

    fn set_material(&mut self, material: Arc<dyn Material>);

    fn get_material(&self) -> Arc<dyn Material> {
        panic!("get_material() not implemented for this Primitive");
    }

    fn get_bounds(&self) -> Bounds;

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
    fn intersect(&self, ray: &mut Ray, surface_interaction: &mut SurfaceInteraction) -> bool {
        let mut t_hit = f32::INFINITY;
        if !self.shape.intersect(&ray, &mut t_hit, surface_interaction) {
            return false;
        }
        ray.t_max = t_hit;

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

    fn get_material(&self) -> Arc<dyn Material> {
        return self
            .material
            .as_ref()
            .expect("no material available")
            .clone();
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
    fn intersect(&self, ray: &mut Ray, surface_interaction: &mut SurfaceInteraction) -> bool {
        let inverse_transform = self.transform.inverse();
        let mut inverse_ray = (inverse_transform)(ray.clone());
        let inverse_t = (inverse_transform)(ray.d).length();

        inverse_ray.d /= inverse_t;
        inverse_ray.t_max *= inverse_t;

        let mut si_primitive = SurfaceInteraction::default();
        if !self
            .primitive
            .intersect(&mut inverse_ray, &mut si_primitive)
        {
            return false;
        }

        ray.t_max = inverse_ray.t_max / inverse_t;

        *surface_interaction = (self.transform)(si_primitive);

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

    fn get_material(&self) -> Arc<dyn Material> {
        return self.primitive.get_material();
    }

    fn get_bounds(&self) -> Bounds {
        return (self.transform)(self.primitive.get_bounds());
    }

    fn get_area(&self) -> f32 {
        return self.primitive.get_area() * self.transform.determinant();
    }

    fn sample(&self) -> (Point, Vector3) {
        let (p, v) = self.primitive.sample();
        return ((self.transform)(p), (self.transform)(v));
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
