use crate::pbrt::*;

// QuadricIntersection Definition

pub struct QuadricIntersection {
    pub tHit: Float,
    pub pObj: Point3f,
    pub phi: Float,
}
