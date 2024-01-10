use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct DielectricBxDF {
    eta: f64,
    mf_distribution: TrowbridgeReitzDistribution,
}

impl BxDF for DielectricBxDF {
    fn fork(&self) -> Arc<dyn BxDF> {
        return Arc::new(self.clone());
    }

    fn flags(&self) -> BxDFFlags {
        let _flags = if self.eta == 1.0 {
            BxDFFlags::Transmission as isize
        } else {
            BxDFFlags::Reflection | BxDFFlags::Transmission
        };

        let flags = _flags
            | (if self.mf_distribution.effectively_smooth() {
                BxDFFlags::Specular
            } else {
                BxDFFlags::Glossy
            }) as isize;

        return BxDFFlags::from(flags);
    }

    fn f(&self, wo: Vector3f, wi: Vector3f, mode: TransportMode) -> SampledSpectrum {
        if self.eta == 1.0 || self.mf_distribution.effectively_smooth() {
            return SampledSpectrum::same_value(0.0);
        }
        // Evaluate rough dielectric BSDF
        // Compute generalized half vector _wm_
        let cos_theta_o = wo.cos_theta();
        let cos_theta_i = wi.cos_theta();
        let reflect = cos_theta_i * cos_theta_o > 0.0;

        let etap = if reflect {
            1.0
        } else {
            if cos_theta_o > 0.0 {
                self.eta
            } else {
                1.0 / self.eta
            }
        };

        let mut wm = wi * etap + wo;
        if cos_theta_i == 0.0 || cos_theta_o == 0.0 || wm.length_squared() == 0.0 {
            return SampledSpectrum::same_value(0.0);
        }

        wm = wm.face_forward(Vector3::new(0.0, 0.0, 1.0)).normalize();

        // Discard backfacing microfacets
        if wm.dot(wi) * cos_theta_i < 0.0 || wm.dot(wo) * cos_theta_o < 0.0 {
            return SampledSpectrum::same_value(0.0);
        }

        let F = fr_dielectric(wo.dot(wm), self.eta);
        if reflect {
            // Compute reflection at rough dielectric interface
            let val = self.mf_distribution.d1(wm) * self.mf_distribution.g(wo, wi) * F
                / (4.0 * cos_theta_i * cos_theta_o).abs();
            return SampledSpectrum::same_value(val);
        }

        // Compute transmission at rough dielectric interface
        let denom = sqr(wi.dot(wm) + wo.dot(wm) / etap) * cos_theta_i * cos_theta_o;
        let mut ft = self.mf_distribution.d1(wm)
            * (1.0 - F)
            * self.mf_distribution.g(wo, wi)
            * (wi.dot(wm) * wo.dot(wm) / denom).abs();

        // Account for non-symmetry with transmission to different medium
        if mode == TransportMode::Radiance {
            ft /= sqr(etap);
        }

        return SampledSpectrum::same_value(ft);
    }

    fn sample_f(
        &self,
        wo: Vector3f,
        uc: f64,
        u: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Option<BSDFSample> {
        if self.eta == 1.0 || self.mf_distribution.effectively_smooth() {
            // Sample perfect specular dielectric BSDF
            let R = fr_dielectric(wo.cos_theta(), self.eta);
            let T = 1.0 - R;
            // Compute probabilities _pr_ and _pt_ for sampling reflection and transmission

            let mut pr = R;
            let mut pt = T;
            if !(sample_flags & BxDFReflTransFlags::Reflection).is_set() {
                pr = 0.0;
            }
            if !(sample_flags & BxDFReflTransFlags::Transmission).is_set() {
                pt = 0.0;
            }
            if pr == 0.0 && pt == 0.0 {
                return None;
            }

            if uc < pr / (pr + pt) {
                // Sample perfect specular dielectric BRDF
                let wi = Vector3f::new(-wo.x, -wo.y, wo.z);
                let fr = R / wi.abs_cos_theta();
                return Some(BSDFSample {
                    f: SampledSpectrum::same_value(fr),
                    wi,
                    pdf: pr / (pr + pt),
                    flags: BxDFFlags::SpecularReflection,
                    eta: 1.0,
                    pdf_is_proportional: false,
                });
            }
            // Sample perfect specular dielectric BTDF
            // Compute ray direction for specular transmission
            let (valid, etap, wi) = refract(wo, Normal3f::new(0.0, 0.0, 1.0), self.eta);
            if !valid {
                return None;
            }

            let mut ft = T / wi.abs_cos_theta();
            // Account for non-symmetry with transmission to different medium
            if mode == TransportMode::Radiance {
                ft /= sqr(etap);
            }

            return Some(BSDFSample {
                f: SampledSpectrum::same_value(ft),
                wi,
                pdf: pt / (pr + pt),
                flags: BxDFFlags::SpecularTransmission,
                eta: etap,
                pdf_is_proportional: false,
            });
        }
        // Sample rough dielectric BSDF
        let wm = self.mf_distribution.sample_wm(wo, u);
        let R = fr_dielectric(wo.dot(wm), self.eta);
        let T = 1.0 - R;

        // Compute probabilities _pr_ and _pt_ for sampling reflection and transmission
        let mut pr = R;
        let mut pt = T;
        if !(sample_flags & BxDFReflTransFlags::Reflection).is_set() {
            pr = 0.0;
        }
        if !(sample_flags & BxDFReflTransFlags::Transmission).is_set() {
            pt = 0.0;
        }
        if pr == 0.0 && pt == 0.0 {
            return None;
        }

        if uc < pr / (pr + pt) {
            // Sample reflection at rough dielectric interface
            let wi = reflect(wo, wm);
            if !wo.same_hemisphere(wi) {
                return None;
            }
            // Compute PDF of rough dielectric reflection
            let pdf = self.mf_distribution.pdf(wo, wm) / (4.0 * wo.abs_dot(wm)) * pr / (pr + pt);
            let f = self.mf_distribution.d1(wm) * self.mf_distribution.g(wo, wi) * R
                / (4.0 * wi.cos_theta() * wo.cos_theta());

            return Some(BSDFSample {
                f: SampledSpectrum::same_value(f),
                wi,
                pdf,
                flags: BxDFFlags::GlossyReflection,
                eta: 1.0,
                pdf_is_proportional: false,
            });
        }
        // Sample transmission at rough dielectric interface
        let (result, etap, wi) = refract(wo, Normal3f::from(wm), self.eta);
        let tir = !result;
        if wo.same_hemisphere(wi) || wi.z == 0.0 || tir {
            return None;
        }

        // Compute PDF of rough dielectric transmission
        let denom = sqr(wi.dot(wm) + wo.dot(wm) / etap);
        let dwm_dwi = wi.abs_dot(wm) / denom;
        let pdf = self.mf_distribution.pdf(wo, wm) * dwm_dwi * pt / (pr + pt);

        // Evaluate BRDF and return _BSDFSample_ for rough transmission
        let mut ft = T
            * self.mf_distribution.d1(wm)
            * self.mf_distribution.g(wo, wi)
            * (wi.dot(wm) * wo.dot(wm) / (wi.cos_theta() * wo.cos_theta() * denom)).abs();

        // Account for non-symmetry with transmission to different medium
        if mode == TransportMode::Radiance {
            ft /= sqr(etap);
        }

        return Some(BSDFSample {
            f: SampledSpectrum::same_value(ft),
            wi,
            pdf,
            flags: BxDFFlags::GlossyTransmission,
            eta: etap,
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
        if self.eta == 1.0 || self.mf_distribution.effectively_smooth() {
            return 0.0;
        }
        // Evaluate sampling PDF of rough dielectric BSDF
        // Compute generalized half vector _wm_
        let cos_theta_o = wo.cos_theta();
        let cos_theta_i = wi.cos_theta();
        let reflect = cos_theta_i * cos_theta_o > 0.0;

        let mut etap = 1.0;
        if !reflect {
            etap = if cos_theta_o > 0.0 {
                self.eta
            } else {
                1.0 / self.eta
            }
        }

        let wm = wi * etap + wo;
        if cos_theta_i == 0.0 || cos_theta_o == 0.0 || wm.length_squared() == 0.0 {
            return 0.0;
        }

        let wm = wm.face_forward(Vector3f::new(0.0, 0.0, 1.0)).normalize();
        // Discard backfacing microfacets
        if wm.dot(wi) * cos_theta_i < 0.0 || wm.dot(wo) * cos_theta_o < 0.0 {
            return 0.0;
        }

        // Determine Fresnel reflectance of rough dielectric boundary
        let R = fr_dielectric(wo.dot(wm), self.eta);
        let T = 1.0 - R;

        // Compute probabilities _pr_ and _pt_ for sampling reflection and transmission
        let mut pr = R;
        let mut pt = T;

        if !(sample_flags & BxDFReflTransFlags::Reflection).is_set() {
            pr = 0.0;
        }
        if !(sample_flags & BxDFReflTransFlags::Transmission).is_set() {
            pt = 0.0;
        }
        if pr == 0.0 && pt == 0.0 {
            return 0.0;
        }

        if reflect {
            // Compute PDF of rough dielectric reflection
            return self.mf_distribution.pdf(wo, wm) / (4.0 * wo.abs_dot(wm)) * pr / (pr + pt);
        }

        // Compute PDF of rough dielectric transmission
        let denom = sqr(wi.dot(wm) + wo.dot(wm) / etap);
        let dwm_dwi = wi.abs_dot(wm) / denom;

        return self.mf_distribution.pdf(wo, wm) * dwm_dwi * pt / (pr + pt);
    }
}

impl DielectricBxDF {
    pub fn new(eta: f64, mf_distribution: TrowbridgeReitzDistribution) -> Self {
        return Self {
            eta,
            mf_distribution,
        };
    }
}
