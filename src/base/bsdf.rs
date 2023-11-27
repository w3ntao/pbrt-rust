use crate::pbrt::*;

pub struct BSDF {
    bxdf: Arc<dyn BxDF>,
    shading_frame: Frame,
}

impl BSDF {
    pub fn new(ns: Normal3f, dpdus: Vector3f, bxdf: Arc<dyn BxDF>) -> Self {
        return Self {
            bxdf,
            shading_frame: Frame::from_xz(dpdus.normalize(), Vector3f::from(ns)),
        };
    }
}
