use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct DiffuseBxDF {
    r: SampledSpectrum,
}

impl DiffuseBxDF {
    pub fn new(r: SampledSpectrum) -> DiffuseBxDF {
        return DiffuseBxDF { r };
    }
}

impl BxDF for DiffuseBxDF {
    fn fork(&self) -> Arc<dyn BxDF> {
        return Arc::new(self.clone());
    }

    fn flags(&self) -> BxDFFlags {
        return if self.r.is_positive() {
            BxDFFlags::DiffuseReflection
        } else {
            BxDFFlags::Unset
        };
    }

    fn f(&self, wo: Vector3f, wi: Vector3f, mode: TransportMode) -> SampledSpectrum {
        if !wo.same_hemisphere(wi) {
            return SampledSpectrum::same_value(0.0);
        }

        return self.r * INV_PI;
    }

    fn sample_f(
        &self,
        wo: Vector3f,
        uc: f64,
        u: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Option<BSDFSample> {
        if !(sample_flags & BxDFReflTransFlags::Reflection).is_set() {
            return None;
        }

        // Sample cosine-weighted hemisphere to compute _wi_ and _pdf_
        let unsigned_wi = sample_cosine_hemisphere(u);
        let wi = if wo.z < 0.0 {
            -unsigned_wi
        } else {
            unsigned_wi
        };

        let pdf = cosine_hemisphere_pdf(wi.abs_cos_theta());

        return Some(BSDFSample {
            f: self.r * INV_PI,
            wi,
            pdf,
            flags: BxDFFlags::DiffuseReflection,
            eta: 1.0,
            pdf_is_proportional: false,
        });
    }

    fn pdf(
        &self,
        wo: Vector3f,
        wi: Vector3f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> f64 {
        if !(sample_flags & BxDFReflTransFlags::Reflection).is_set() || !wo.same_hemisphere(wi) {
            return 0.0;
        }

        return cosine_hemisphere_pdf(wi.abs_cos_theta());
    }
}
