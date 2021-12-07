use std::rc::Rc;
use crate::ray_tracing::primitive::Primitive;

pub trait Group {
    fn add(&mut self, p: Rc<dyn Primitive>);
}

