use crate::pbrt::*;

pub struct SpectrumConstantTexture {
    value: Arc<dyn Spectrum>,
}

impl SpectrumConstantTexture {
    pub fn new(spectrum: Arc<dyn Spectrum>) -> Self {
        return Self {
            value: spectrum,
        };
    }
}

impl SpectrumTexture for SpectrumConstantTexture {
    fn evaluate(&self, ctx: &TextureEvalContext, lambda: &SampledWavelengths) -> SampledSpectrum {
        return self.value.sample(lambda);
    }
}
