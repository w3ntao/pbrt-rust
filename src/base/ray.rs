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

pub struct DifferentialRay {
    pub ray: Ray,

    pub has_differentials: bool,
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vector3f,
    pub ry_direction: Vector3f,
}

impl DifferentialRay {
    pub fn new(o: Point3f, d: Vector3f) -> Self {
        return Self {
            ray: Ray::new(o, d),
            has_differentials: false,
            rx_origin: Point3f::nan(),
            ry_origin: Point3f::nan(),
            rx_direction: Vector3::nan(),
            ry_direction: Vector3::nan(),
        };
    }
}

impl DifferentialRay {
    fn at(&self, t: Float) -> Point3f {
        return self.ray.at(t);
    }
}
