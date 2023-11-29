use crate::pbrt::*;

pub struct DiffuseMaterial {
    reflectance: Arc<dyn SpectrumTexture>,
}

impl Material for DiffuseMaterial {
    fn get_bsdf(&self, context: &MaterialEvalContext, lambda: &SampledWavelengths) -> BSDF {
        let r = self
            .reflectance
            .evaluate(&context.texture_eval_context, lambda);

        return BSDF::new(
            context.ns,
            context.dpdus,
            Some(Arc::new(DiffuseBxDF::new(r))),
        );
    }
}

impl DiffuseMaterial {
    pub fn new(reflectance: Arc<dyn SpectrumTexture>) -> Self {
        return Self { reflectance };
    }
}
