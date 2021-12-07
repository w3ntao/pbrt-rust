use std::rc::Rc;
use crate::ray_tracing::primitive::Primitive;

pub struct World {
    pub scene: Rc<dyn Primitive>,
}

impl World {
    pub fn new(_scene: Rc<dyn Primitive>) -> Self {
        return Self {
            scene: _scene,
        };
    }
}
