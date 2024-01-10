use crate::pbrt::*;

pub struct PhaseFunctionSample {
    pub p: f64,
    pub wi: Vector3f,
    pub pdf: f64,
}

pub trait PhaseFunction {}

pub struct HGPhaseFunction {
    g: f64,
}

impl HGPhaseFunction {
    pub fn new(g: f64) -> Self {
        return Self { g };
    }

    pub fn p(&self, wo: Vector3f, wi: Vector3f) -> f64 {
        return henyey_greenstein(wo.dot(wi), self.g);
    }

    pub fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f64 {
        return self.p(wo, wi);
    }

    pub fn sample_p(&self, wo: Vector3f, u: Point2f) -> Option<PhaseFunctionSample> {
        let (wi, pdf) = sample_henyey_greenstein(wo, self.g, u);

        return Some(PhaseFunctionSample { p: pdf, wi, pdf });
    }
}
