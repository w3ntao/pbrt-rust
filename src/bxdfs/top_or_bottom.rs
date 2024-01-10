use crate::pbrt::*;

pub struct TopOrBottomBxDF<TypeTopBxDF: BxDF, TypeBottomBxDF: BxDF> {
    pub top_bxdf: Option<Arc<TypeTopBxDF>>,
    pub bottom_bxdf: Option<Arc<TypeBottomBxDF>>,
}

impl<TypeTopBxDF: BxDF, TypeBottomBxDF: BxDF> TopOrBottomBxDF<TypeTopBxDF, TypeBottomBxDF> {
    pub fn new(
        top_bxdf: Option<Arc<TypeTopBxDF>>,
        bottom_bxdf: Option<Arc<TypeBottomBxDF>>,
    ) -> Self {
        return Self {
            top_bxdf,
            bottom_bxdf,
        };
    }

    pub fn flags(&self) -> BxDFFlags {
        return match (&self.top_bxdf, &self.bottom_bxdf) {
            (Some(top_bxdf), _) => top_bxdf.flags(),

            (None, Some(bottom_bxdf)) => bottom_bxdf.flags(),

            (None, None) => {
                panic!("both top and bottom BxDF is None");
            }
        };
    }

    pub fn f(&self, wo: Vector3f, wi: Vector3f, mode: TransportMode) -> SampledSpectrum {
        return match (&self.top_bxdf, &self.bottom_bxdf) {
            (Some(top_bxdf), _) => top_bxdf.f(wo, wi, mode),

            (None, Some(bottom_bxdf)) => bottom_bxdf.f(wo, wi, mode),

            (None, None) => {
                panic!("both top and bottom BxDF is None");
            }
        };
    }

    pub fn pdf(
        &self,
        wo: Vector3f,
        wi: Vector3f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
        // default args: BxDFReflTransFlags sampleFlags = BxDFReflTransFlags::All
    ) -> f64 {
        return match (&self.top_bxdf, &self.bottom_bxdf) {
            (Some(top_bxdf), _) => top_bxdf.pdf(wo, wi, mode, sample_flags),

            (None, Some(bottom_bxdf)) => bottom_bxdf.pdf(wo, wi, mode, sample_flags),

            (None, None) => {
                panic!("both top and bottom BxDF is None");
            }
        };
    }

    pub fn sample_f(
        &self,
        wo: Vector3f,
        uc: f64,
        u: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Option<BSDFSample> {
        return match (&self.top_bxdf, &self.bottom_bxdf) {
            (Some(top_bxdf), _) => top_bxdf.sample_f(wo, uc, u, mode, sample_flags),

            (None, Some(bottom_bxdf)) => bottom_bxdf.sample_f(wo, uc, u, mode, sample_flags),

            (None, None) => {
                panic!("both top and bottom BxDF is None");
            }
        };
    }
}
