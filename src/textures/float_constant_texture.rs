use crate::pbrt::*;

pub struct FloatConstantTexture {
    value: Float,
}

impl FloatTexture for FloatConstantTexture {
    fn evaluate(&self, ctx: &TextureEvalContext) -> Float {
        return self.value;
    }
}
