use crate::pbrt::*;

pub struct SpectrumScaledTexture {
    texture: Arc<dyn SpectrumTexture>,
    scale: f64,
    // TODO: change f64 to FloatTexture
}

impl SpectrumTexture for SpectrumScaledTexture {
    fn evaluate(&self, ctx: &TextureEvalContext, lambda: &SampledWavelengths) -> SampledSpectrum {
        if self.scale == 0.0 {
            return SampledSpectrum::same_value(0.0);
        }

        return self.texture.evaluate(ctx, lambda) * self.scale;
    }
}

impl SpectrumScaledTexture {
    pub fn new(texture: Arc<dyn SpectrumTexture>, scale: f64) -> Self {
        return Self { texture, scale };
    }
}
