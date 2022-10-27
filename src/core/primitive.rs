use crate::core::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, t_min: f32, interaction: &mut SurfaceInteraction) -> bool;

    fn get_bounds(&self) -> Bounds;

    fn set_material(&mut self, material: Arc<dyn Material>);

    fn sample(&self) -> (Point, Vector3) {
        panic!("sample() not implemented for this Primitive");
    }

    fn get_area(&self) -> f32 {
        panic!("get_area() not implemented for this Primitive");
    }
}

pub trait Aggregate {
    fn add(&mut self, p: Arc<dyn Primitive>);
}
