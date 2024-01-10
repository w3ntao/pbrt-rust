use crate::pbrt::*;

#[derive(Clone)]
pub struct CoatedDiffuseBxDF {
    bxdf: LayeredBxDF<DielectricBxDF, DiffuseBxDF, true>,
}

impl BxDF for CoatedDiffuseBxDF {
    fn fork(&self) -> Arc<dyn BxDF> {
        return Arc::new(self.clone());
    }

    fn flags(&self) -> BxDFFlags {
        return self.bxdf.flags();
    }

    fn f(&self, wo: Vector3f, wi: Vector3f, mode: TransportMode) -> SampledSpectrum {
        return self.bxdf.f(wo, wi, mode);
    }

    fn sample_f(
        &self,
        wo: Vector3f,
        uc: f64,
        u: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Option<BSDFSample> {
        return self.bxdf.sample_f(wo, uc, u, mode, sample_flags);
    }

    fn pdf(
        &self,
        wo: Vector3f,
        wi: Vector3f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> f64 {
        return self.bxdf.pdf(wo, wi, mode, sample_flags);
    }
}

impl CoatedDiffuseBxDF {
    pub fn new(
        top_bxdf: Arc<DielectricBxDF>,
        bottom_bxdf: Arc<DiffuseBxDF>,
        thickness: f64,
        albedo: SampledSpectrum,
        g: f64,
        max_depth: usize,
        n_samples: usize,
    ) -> Self {
        return Self {
            bxdf: LayeredBxDF::<DielectricBxDF, DiffuseBxDF, true>::new(
                top_bxdf,
                bottom_bxdf,
                thickness,
                albedo,
                g,
                max_depth,
                n_samples,
            ),
        };
    }
}
