use crate::pbrt::*;

pub struct BSDF {
    pub bxdf: Option<Arc<dyn BxDF>>,
    // TODO: should I rewrite bxdf to Arc<dyn BxDF>?
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
        /*
        default arguments:
            mode = TransportMode::Radiance
        */
        let wi = self.render_to_local(wi_render);
        let wo = self.render_to_local(wo_render);

        if wo.z == 0.0 {
            return SampledSpectrum::same_value(0.0);
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

    pub fn sample_f(
        &self,
        wo_render: Vector3f,
        u: Float,
        u2: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Option<BSDFSample> {
        /*
        default arguments:
            mode: TransportMode::Radiance
            sample_flags: BxDFReflTransFlags::All
        */

        let wo = self.render_to_local(wo_render);
        let bxdf_flags = match &self.bxdf {
            None => {
                panic!("BSDF::sample_f(): bxdf shouldn't be None");
            }
            Some(bxdf) => bxdf.flags(),
        };

        if wo.z == 0.0 || !((bxdf_flags as usize) & (sample_flags as usize) > 0) {
            return None;
        }

        // Sample _bxdf_ and return _BSDFSample_
        let mut bs = {
            let option_bs = match &self.bxdf {
                None => {
                    panic!("BSDF::sample_f(): bxdf shouldn't be None");
                }
                Some(_bxdf) => _bxdf.sample_f(wo, u, u2, mode, sample_flags),
            };
            match option_bs {
                None => {
                    return None;
                }
                Some(_bs) => _bs,
            }
        };

        if !bs.f.is_positive() || bs.pdf == 0.0 || bs.wi.z == 0.0 {
            return None;
        }

        bs.wi = self.local_to_render(bs.wi);
        return Some(bs);
    }
}
