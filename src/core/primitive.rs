use crate::core::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction>;

    fn set_material(&mut self, material: Arc<dyn Material>);

    fn get_material(&self) -> Arc<dyn Material> {
        panic!("get_material() not implemented for this Primitive");
    }

    fn get_bounds(&self) -> Bounds;

    fn get_area(&self) -> f32;

    fn sample(&self, _sampler: &mut dyn Sampler) -> (Point, Vector3) {
        panic!("sample() is not implemented for this Primitive");
    }
}

pub trait Aggregate {
    fn add(&mut self, p: Arc<dyn Primitive>);
}

pub struct GeometricPrimitive {
    shape: Arc<dyn Shape>,
    material: Option<Arc<dyn Material>>,
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        let mut t_hit = f32::INFINITY;

        return match self.shape.intersect(&ray, &mut t_hit) {
            None => None,
            Some(mut si) => {
                ray.t_max = t_hit;

                match &self.material {
                    Some(material) => {
                        si.material = Some(material.clone());
                    }
                    _ => {}
                }

                Some(si)
            }
        };
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

    fn sample(&self, sampler: &mut dyn Sampler) -> (Point, Vector3) {
        return self.shape.sample(sampler);
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
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        let inverse_transform = self.transform.inverse();
        let mut inverse_ray = inverse_transform.on_ray(ray.clone());

        let rescaling = {
            let direction_length = inverse_transform.on_vector(ray.d).length();
            2.0_f32.powi(direction_length.log2().round() as i32)
        };
        // rescaling with 2^x to minimise computational errors
        // introduced by division and multiplication

        inverse_ray.d /= rescaling;
        inverse_ray.t_max *= rescaling;

        return match self.primitive.intersect(&mut inverse_ray) {
            None => None,
            Some(inverse_si) => {
                ray.t_max = inverse_ray.t_max / rescaling;

                let mut surface_interaction = self.transform.on_surface_interaction(inverse_si);

                match &self.material {
                    Some(material) => {
                        surface_interaction.material = Some(material.clone());
                    }
                    _ => {}
                }

                Some(surface_interaction)
            }
        };
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = Some(material);
    }

    fn get_material(&self) -> Arc<dyn Material> {
        return self.primitive.get_material();
    }

    fn get_bounds(&self) -> Bounds {
        return self.transform.on_bounds(self.primitive.get_bounds());
    }

    fn get_area(&self) -> f32 {
        return self.primitive.get_area() * self.transform.determinant();
    }

    fn sample(&self, sampler: &mut dyn Sampler) -> (Point, Vector3) {
        let (p, v) = self.primitive.sample(sampler);
        return (self.transform.on_point(p), self.transform.on_vector(v));
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
