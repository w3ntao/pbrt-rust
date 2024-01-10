use crate::pbrt::*;

pub struct CoatedDiffuseMaterial {
    reflectance: Arc<dyn SpectrumTexture>,
    albedo: Arc<dyn SpectrumTexture>,
    u_roughness: Arc<dyn FloatTexture>,
    v_roughness: Arc<dyn FloatTexture>,
    thickness: Arc<dyn FloatTexture>,
    g: Arc<dyn FloatTexture>,
    eta: Arc<dyn Spectrum>,
    remap_roughness: bool,
    max_depth: usize,
    n_samples: usize,
    // TODO: displacement and normal_map not implemented
}

impl Material for CoatedDiffuseMaterial {
    fn get_bsdf(&self, ctx: &MaterialEvalContext, lambda: &mut SampledWavelengths) -> BSDF {
        let r = self
            .reflectance
            .evaluate(&ctx.texture_eval_context, lambda)
            .clamp(0.0, 1.0);

        let u_rough = self.u_roughness.evaluate(&ctx.texture_eval_context);
        let v_rough = self.v_roughness.evaluate(&ctx.texture_eval_context);

        if self.remap_roughness {
            panic!("this part is not implemented");
        }

        let distribution = TrowbridgeReitzDistribution::new(u_rough, v_rough);

        let thick = self.thickness.evaluate(&ctx.texture_eval_context);

        let mut sampled_eta = self.eta.eval(lambda[0]);

        if !self.eta.is_constant_spectrum() {
            lambda.terminate_secondary();
        }

        if sampled_eta == 0.0 {
            sampled_eta = 1.0;
        }

        let a = self
            .albedo
            .evaluate(&ctx.texture_eval_context, lambda)
            .clamp(0.0, 1.0);

        let gg = self.g.evaluate(&ctx.texture_eval_context).clamp(-1.0, 1.0);

        let coated_diffuse_bxdf = CoatedDiffuseBxDF::new(
            Arc::new(DielectricBxDF::new(sampled_eta, distribution)),
            Arc::new(DiffuseBxDF::new(r)),
            thick,
            a,
            gg,
            self.max_depth,
            self.n_samples,
        );

        return BSDF::new(ctx.ns, ctx.dpdus, Some(Arc::new(coated_diffuse_bxdf)));
    }
}

impl CoatedDiffuseMaterial {
    pub fn new(
        reflectance: Arc<dyn SpectrumTexture>,
        albedo: Arc<dyn SpectrumTexture>,
        u_roughness: Arc<dyn FloatTexture>,
        v_roughness: Arc<dyn FloatTexture>,
        thickness: Arc<dyn FloatTexture>,
        g: Arc<dyn FloatTexture>,
        eta: Arc<dyn Spectrum>,
        remap_roughness: bool,
        max_depth: usize,
        n_samples: usize,
    ) -> Self {
        return Self {
            reflectance,
            albedo,
            u_roughness,
            v_roughness,
            thickness,
            g,
            eta,
            remap_roughness,
            max_depth,
            n_samples,
        };
    }
}
