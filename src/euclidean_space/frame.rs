use crate::pbrt::*;

pub struct Frame {
    pub x: Vector3f,
    pub y: Vector3f,
    pub z: Vector3f,
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ Frame [x: {} y: {} z: {}] ]", self.x, self.y, self.z)
    }
}

impl Frame {
    pub fn nan() -> Self {
        return Self {
            x: Vector3f::nan(),
            y: Vector3f::nan(),
            z: Vector3f::nan(),
        };
    }
    pub fn from_z(z: Vector3f) -> Self {
        let (x, y) = z.coordinate_system();

        return Self { x, y, z };
    }

    pub fn from_xz(x: Vector3f, z: Vector3f) -> Self {
        return Self {
            x,
            y: z.cross(x),
            z,
        };
    }

    pub fn to_local(&self, v: Vector3f) -> Vector3f {
        return Vector3f {
            x: v.dot(self.x),
            y: v.dot(self.y),
            z: v.dot(self.z),
        };
    }

    pub fn from_local(&self, v: Vector3f) -> Vector3f {
        return v.x * self.x + v.y * self.y + v.z * self.z;
    }
}
