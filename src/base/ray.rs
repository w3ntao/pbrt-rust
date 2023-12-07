use crate::pbrt::*;

#[derive(Clone)]
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

fn offset_ray_origin(pi: Point3fi, n: Normal3f, w: Vector3f) -> Point3f {
    // Find vector _offset_ to corner of error bounds and compute initial _po_

    let n_as_vec3 = Vector3f::from(n);
    let d = n_as_vec3.abs().dot(pi.error());
    let offset = {
        let _offset = d * n_as_vec3;
        if w.dot(n_as_vec3) < 0.0 {
            -_offset
        } else {
            _offset
        }
    };

    let mut po = Point3f::from(pi) + offset;

    // Round offset point _po_ away from _p_
    for i in 0..3 {
        if offset[i] > 0.0 {
            po[i] = next_float_up(po[i]);
        } else if offset[i] < 0.0 {
            po[i] = next_float_down(po[i]);
        }
    }

    return po;
}

pub fn spawn_ray_to(p_from: Point3fi, n_from: Normal3f, p_to: Point3fi, n_to: Normal3f) -> Ray {
    let pf = offset_ray_origin(p_from, n_from, Point3f::from(p_to) - Point3f::from(p_from));

    let pt = offset_ray_origin(p_to, n_to, pf - Point3f::from(p_to));

    return Ray::new(pf, pt - pf);
}

#[derive(Clone)]
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
