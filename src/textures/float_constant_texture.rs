use crate::pbrt::*;

pub struct FloatConstantTexture {
    value: f64,
}

impl FloatTexture for FloatConstantTexture {
    fn evaluate(&self, ctx: &TextureEvalContext) -> f64 {
        return self.value;
    }
}

impl FloatConstantTexture {
    pub fn new(value: f64) -> Self {
        return Self { value };
    }
}
