use std::sync::Arc;
use crate::ray_tracing::primitive::Primitive;

pub trait Group {
    fn add(&mut self, p: Arc<dyn Primitive>);
}

