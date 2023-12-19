use crate::pbrt::*;

#[derive(Copy, Clone, PartialEq)]
pub enum LightType {
    DeltaPosition,
    DeltaDirection,
    Area,
    Infinite,
}

pub struct LightSampleContext {
    pub pi: Point3fi,
    pub n: Normal3f,
    pub ns: Normal3f,
}

impl LightSampleContext {
    pub fn from_surface_interaction(si: &SurfaceInteraction) -> Self {
        return Self {
            pi: si.interaction.pi,
            n: si.interaction.n,
            ns: si.shading.n,
        };
    }
}

pub struct LightLiSample {
    pub l: SampledSpectrum,
    pub wi: Vector3f,
    pub pdf: f64,
    pub p_light: Interaction,
}

pub struct LightBase {
    pub light_type: LightType,
    pub render_from_light: Transform,
}

pub trait Light: Send + Sync {
    fn light_type(&self) -> LightType;

    fn le(&self, ray: &Ray, lambda: &SampledWavelengths) -> SampledSpectrum;

    fn l(
        &self,
        p: Point3f,
        n: Normal3f,
        uv: Point2f,
        w: Vector3f,
        lambda: &SampledWavelengths,
    ) -> SampledSpectrum {
        panic!("Light::l() can only be invoked by AreaLight");
    }

    fn sample_li(
        &self,
        ctx: &LightSampleContext,
        u: Point2f,
        lambda: &SampledWavelengths,
        allow_incomplete_pdf: bool,
    ) -> Option<LightLiSample>;
}
