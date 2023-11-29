use crate::pbrt::*;

pub struct BSDF {
    pub bxdf: Option<Arc<dyn BxDF>>,
    pub shading_frame: Frame,
}

impl BSDF {
    pub fn new(ns: Normal3f, dpdus: Vector3f, bxdf: Option<Arc<dyn BxDF>>) -> Self {
        return Self {
            bxdf,
            shading_frame: Frame::from_xz(dpdus.normalize(), Vector3f::from(ns)),
        };
    }

    fn render_to_local(&self, v: Vector3f) -> Vector3f {
        return self.shading_frame.to_local(v);
    }

    fn local_to_render(&self, v: Vector3f) -> Vector3f {
        return self.shading_frame.from_local(v);
    }
    
    pub fn f(
        &self,
        wo_render: Vector3f,
        wi_render: Vector3f,
        mode: TransportMode,
    ) -> SampledSpectrum {
        let wi = self.render_to_local(wi_render);
        let wo = self.render_to_local(wo_render);

        if wo.z == 0.0 {
            return SampledSpectrum::zero();
        }

        match &self.bxdf {
            None => {
                panic!("can't evaluate void bxdf");
            }
            Some(bxdf) => {
                return bxdf.f(wo, wi, mode);
            }
        };
    }
}
