use crate::core::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &mut Ray, surface_interaction: &mut SurfaceInteraction) -> bool;

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
    fn intersect(&self, ray: &mut Ray, surface_interaction: &mut SurfaceInteraction) -> bool {
        let inverse_transform = self.transform.inverse();
        let inverse_dir = (inverse_transform)(ray.d);
        let inverse_t = inverse_dir.length();
        let mut inverse_ray = Ray::new(
            (inverse_transform)(ray.o),
            inverse_dir.normalize(),
            ray.t_min * inverse_t,
            ray.t_max * inverse_t,
        );

        if !self
            .primitive
            .intersect(&mut inverse_ray, surface_interaction)
        {
            return false;
        }

        ray.t_min = inverse_ray.t_min / inverse_t;
        ray.t_max = inverse_ray.t_max / inverse_t;

        if !self.transform.is_identity() {
            surface_interaction.n = (self.transform)(surface_interaction.n);
            surface_interaction.p = (self.transform)(surface_interaction.p);
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
        return (self.transform)(self.primitive.get_bounds());
    }

    fn get_area(&self) -> f32 {
        let area = self.primitive.get_area();
        if self.transform.is_identity() {
            return area;
        }
        return area * self.transform.determinant();
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
