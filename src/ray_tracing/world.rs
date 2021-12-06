use crate::ray_tracing::primitive::Primitive;

pub struct World<'a> {
    pub scene: &'a dyn Primitive,
}

impl<'a> World<'a> {
    pub fn new(_scene: &'a dyn Primitive) -> Self {
        return Self {
            scene: _scene,
        };
    }
}
