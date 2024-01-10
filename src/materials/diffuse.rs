use crate::pbrt::*;

pub struct DiffuseMaterial {
    reflectance: Arc<dyn SpectrumTexture>,
}

impl Material for DiffuseMaterial {
    fn get_bsdf(&self, ctx: &MaterialEvalContext, lambda: &mut SampledWavelengths) -> BSDF {
        let r = self
            .reflectance
            .evaluate(&ctx.texture_eval_context, lambda)
            .clamp(0.0, 1.0);

        return BSDF::new(ctx.ns, ctx.dpdus, Some(Arc::new(DiffuseBxDF::new(r))));
    }
}

impl DiffuseMaterial {
    pub fn new(reflectance: Arc<dyn SpectrumTexture>) -> Self {
        return Self { reflectance };
    }
}
