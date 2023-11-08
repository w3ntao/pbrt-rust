use crate::pbrt::*;

pub enum TransportMode {
    Radiance,
    Importance,
}

pub enum BxDFReflTransFlags {
    Unset = 0,
    Reflection = 1 << 0,
    Transmission = 1 << 1,
    All = 1 << 0 | 1 << 1,
}

impl BitOr for BxDFReflTransFlags {
    type Output = bool;

    fn bitor(self, rhs: Self) -> Self::Output {
        return (self as isize) | (rhs as isize) > 0;
    }
}

impl BitAnd for BxDFReflTransFlags {
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        return (self as isize) & (rhs as isize) > 0;
    }
}

pub enum BxDFFlags {
    Unset = 0,
    Reflection = 1 << 0,
    Transmission = 1 << 1,
    Diffuse = 1 << 2,
    Glossy = 1 << 3,
    Specular = 1 << 4,
    // Composite _BxDFFlags_ definitions
    DiffuseReflection = 1 << 2 | 1 << 0,
    DiffuseTransmission = 1 << 2 | 1 << 1,
    GlossyReflection = 1 << 3 | 1 << 0,
    GlossyTransmission = 1 << 3 | 1 << 1,
    SpecularReflection = 1 << 4 | 1 << 0,
    SpecularTransmission = 1 << 4 | 1 << 1,
    All = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

impl BitOr for BxDFFlags {
    type Output = bool;

    fn bitor(self, rhs: Self) -> Self::Output {
        return (self as isize) | (rhs as isize) > 0;
    }
}

pub struct BSDFSample {
    pub f: SampledSpectrum,
    pub wi: Vector3f,
    pub pdf: Float,
    pub flags: BxDFFlags,
    pub eta: Float,
    pub pdf_is_proportional: bool,
}

pub trait BxDF {
    fn flags(&self) -> BxDFFlags;

    fn f(&self, wo: Vector3f, wi: Vector3f, mode: TransportMode) -> SampledSpectrum;

    fn sample_f(
        &self,
        wo: Vector3f,
        uc: Float,
        u: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Option<BSDFSample>;

    fn pdf(
        wo: Vector3f,
        wi: Vector3f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Float;
}
