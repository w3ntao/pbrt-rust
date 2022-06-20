use std::sync::Arc;

use crate::ray_tracing::primitive::Primitive;

pub struct World {
    pub scene: Arc<dyn Primitive>,
    pub lights: Vec<Arc<dyn Primitive>>,
}

impl World {
    pub fn new(_scene: Arc<dyn Primitive>) -> Self {
        return Self {
            scene: _scene,
            lights: vec![],
        };
    }
}
