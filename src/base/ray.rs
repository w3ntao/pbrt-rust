use crate::pbrt::*;

pub struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
}

impl Ray {
    pub fn new(o: Point3f, d: Vector3f) -> Ray {
        return Ray { o, d };
    }

    pub fn at(&self, t: Float) -> Point3f {
        return self.o + t * self.d;
    }
}
