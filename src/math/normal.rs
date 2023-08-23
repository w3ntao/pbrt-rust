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

impl Default for Normal3f {
    fn default() -> Self {
        return Self {
            x: Float::NAN,
            y: Float::NAN,
            z: Float::NAN,
        };
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

impl Display for Normal3f {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ {}, {}, {} ]", self.x, self.y, self.z)
    }
}
