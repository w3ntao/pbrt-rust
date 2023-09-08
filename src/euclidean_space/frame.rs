use crate::pbrt::*;

pub struct Frame {
    pub x: Vector3f,
    pub y: Vector3f,
    pub z: Vector3f,
}

impl Frame {
    pub fn from_z(z: Vector3f) -> Self {
        let (x, y) = z.coordinate_system();

        return Frame { x, y, z };
    }

    pub fn from_local(&self, v: Vector3f) -> Vector3f {
        return v.x * self.x + v.y * self.y + v.z * self.z;
    }
}
