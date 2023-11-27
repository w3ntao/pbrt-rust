use crate::pbrt::*;

pub struct Shading {
    pub n: Normal3f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Vector3f,
    pub dndv: Vector3f,
}

impl Shading {
    pub fn nan() -> Self {
        return Self {
            n: Normal3f::nan(),
            dpdu: Vector3::nan(),
            dpdv: Vector3::nan(),
            dndu: Vector3::nan(),
            dndv: Vector3::nan(),
        };
    }
}

pub struct SurfaceInteraction {
    pub pi: Point3fi,
    pub n: Normal3f,
    pub uv: Point2f,
    pub wo: Vector3f,

    pub dpdx: Vector3f,
    pub dpdy: Vector3f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,

    pub dndu: Normal3f,
    pub dndv: Normal3f,
    pub dudx: Float,
    pub dvdx: Float,
    pub dudy: Float,
    pub dvdy: Float,

    pub shading: Shading,
}

pub fn offset_ray_origin(pi: Point3fi, n: Normal3f, w: Vector3f) -> Point3f {
    // Find vector _offset_ to corner of error bounds and compute initial _po_
    let d = Vector3f::from(n).abs().dot(pi.error());
    let _offset = d * Vector3f::from(n);
    let offset = if n.dot(w) < 0.0 { -_offset } else { _offset };
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

impl SurfaceInteraction {
    pub fn new(
        pi: Point3fi,
        uv: Point2f,
        wo: Vector3f,
        dpdu: Vector3f,
        dpdv: Vector3f,
        dndu: Normal3f,
        dndv: Normal3f,
    ) -> Self {
        return Self {
            pi,
            n: Normal3f::from(dpdu.cross(dpdv).normalize()),
            uv,
            wo: wo.normalize(),
            dpdx: Vector3::nan(),
            dpdy: Vector3::nan(),
            dpdu,
            dpdv,
            dndu,
            dndv,
            dudx: 0.0,
            dvdx: 0.0,
            dudy: 0.0,
            dvdy: 0.0,
            shading: Shading::nan(),
        };
    }

    pub fn offset_ray_origin(&self, w: Vector3f) -> Point3f {
        return offset_ray_origin(self.pi, self.n, w);
    }

    pub fn spawn_ray(&self, d: Vector3f) -> DifferentialRay {
        return DifferentialRay::new(self.offset_ray_origin(d), d);
    }
}

pub struct ShapeIntersection {
    pub t_hit: Float,
    pub interaction: SurfaceInteraction,
}

pub struct QuadricIntersection {
    pub t_hit: Float,
    pub p_obj: Point3f,
    pub phi: Float,
}

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &dyn Ray, t_max: Float) -> Option<ShapeIntersection>;

    fn fast_intersect(&self, ray: &dyn Ray, t_max: Float) -> bool;
    fn bounds(&self) -> Bounds3f;
}
