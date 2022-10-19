use crate::core::primitive::Primitive;
use std::sync::Arc;

pub trait Group {
    fn add(&mut self, p: Arc<dyn Primitive>);
}
