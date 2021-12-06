use std::rc::Rc;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;

pub trait Group {
    fn add(&mut self, p: Rc<dyn Primitive>);
}

