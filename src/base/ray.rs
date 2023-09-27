use crate::pbrt::*;

pub trait Ray {
    fn get_o(&self) -> Point3f;

    fn get_d(&self) -> Vector3f;

    fn at(&self, t: Float) -> Point3f;

    fn to_simple_ray(&self) -> SimpleRay {
        return SimpleRay {
            o: self.get_o(),
            d: self.get_d(),
        };
    }
}

pub struct SimpleRay {
    pub o: Point3f,
    pub d: Vector3f,
}

impl SimpleRay {
    pub fn new(o: Point3f, d: Vector3f) -> SimpleRay {
        return SimpleRay { o, d };
    }

    pub fn at(&self, t: Float) -> Point3f {
        return self.o + t * self.d;
    }
}

impl Ray for SimpleRay {
    fn get_o(&self) -> Point3f {
        return self.o;
    }

    fn get_d(&self) -> Vector3f {
        return self.d;
    }

    fn at(&self, t: Float) -> Point3f {
        return self.o + t * self.d;
    }
}

pub struct DifferentialRay {
    pub o: Point3f,
    pub d: Vector3f,
    pub has_differentials: bool,
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vector3f,
    pub ry_direction: Vector3f,
}

impl DifferentialRay {
    pub fn new(o: Point3f, d: Vector3f) -> Self {
        return Self {
            o,
            d,
            has_differentials: false,
            rx_origin: Point3f::nan(),
            ry_origin: Point3f::nan(),
            rx_direction: Vector3::nan(),
            ry_direction: Vector3::nan(),
        };
    }
}

impl Ray for DifferentialRay {
    fn get_o(&self) -> Point3f {
        return self.o;
    }

    fn get_d(&self) -> Vector3f {
        return self.d;
    }

    fn at(&self, t: Float) -> Point3f {
        return self.o + t * self.d;
    }
}
