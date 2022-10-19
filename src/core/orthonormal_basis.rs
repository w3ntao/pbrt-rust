use crate::core::interfaces::*;

#[derive(Copy, Clone)]
pub struct OrthonormalBasis {
    u: Vector3,
    v: Vector3,
    w: Vector3,
}

impl OrthonormalBasis {
    pub fn build_from_w(n: Vector3) -> Self {
        let _w = n.normalize();
        let a = if _w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };

        let _v = cross(_w, a);

        return OrthonormalBasis {
            u: cross(_w, _v),
            v: _v,
            w: _w,
        };
    }

    pub fn local(&self, v3: Vector3) -> Vector3 {
        return v3.x * self.u + v3.y * self.v + v3.z * self.w;
    }
}
