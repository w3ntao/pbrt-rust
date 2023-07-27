use crate::pbrt::*;

pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Normal3f {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        return Self { x, y, z };
    }
}

impl From<Vector3f> for Normal3f {
    fn from(v: Vector3f) -> Self {
        return Self {
            x: v.x,
            y: v.y,
            z: v.z,
        };
    }
}
