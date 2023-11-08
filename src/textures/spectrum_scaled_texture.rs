use crate::pbrt::*;

pub struct SpectrumScaledTexture {
    texture: Arc<dyn SpectrumTexture>,
    scale: Float,
    // TODO: change Float to FloatTexture
}

impl SpectrumTexture for SpectrumScaledTexture {
    fn evaluate(&self, ctx: &TextureEvalContext, lambda: &SampledWavelengths) -> SampledSpectrum {
        panic!("evaluate() not implemented for SpectrumScaledTexture");
    }
}

impl SpectrumScaledTexture {
    pub fn new(texture: Arc<dyn SpectrumTexture>, scale: Float) -> Self {
        return Self { texture, scale };
    }
}
