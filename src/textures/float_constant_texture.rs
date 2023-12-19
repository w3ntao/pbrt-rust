use crate::pbrt::*;

pub struct FloatConstantTexture {
    value: f64,
}

impl FloatTexture for FloatConstantTexture {
    fn evaluate(&self, ctx: &TextureEvalContext) -> f64 {
        return self.value;
    }
}
