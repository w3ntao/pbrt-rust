use crate::pbrt::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TransportMode {
    Radiance,
    Importance,
}

impl TransportMode {
    pub fn not(&self) -> TransportMode {
        return match *self {
            TransportMode::Radiance => TransportMode::Importance,
            TransportMode::Importance => TransportMode::Radiance,
        };
    }
}

#[derive(Copy, Clone, Eq, EnumIter, Hash, PartialEq)]
pub enum BxDFReflTransFlags {
    Unset = 0,
    Reflection = 1 << 0,
    Transmission = 1 << 1,
    All = 1 << 0 | 1 << 1,
}

lazy_static! {
    static ref reversed_bxdf_refl_trans_flag_map: HashMap<isize, BxDFReflTransFlags> = {
        let mut reversed_map = HashMap::default();

        for flag in BxDFReflTransFlags::iter() {
            reversed_map.insert(flag as isize, flag);
        }

        reversed_map
    };
}

impl From<isize> for BxDFReflTransFlags {
    fn from(value: isize) -> Self {
        return match reversed_bxdf_refl_trans_flag_map.get(&value) {
            None => {
                panic!("illegal value: `{}`", value);
            }
            Some(flag) => *flag,
        };
    }
}

impl BitOr for BxDFReflTransFlags {
    type Output = BxDFReflTransFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        return BxDFReflTransFlags::from((self as isize) | (rhs as isize));
    }
}

impl BitAnd for BxDFReflTransFlags {
    type Output = BxDFReflTransFlags;

    fn bitand(self, rhs: Self) -> Self::Output {
        return BxDFReflTransFlags::from((self as isize) & (rhs as isize));
    }
}

impl BxDFReflTransFlags {
    pub fn is_set(&self) -> bool {
        return *self != BxDFReflTransFlags::Unset;
    }
}

#[derive(Copy, Clone, Debug, Eq, EnumIter, Hash, PartialEq)]
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
    GlossyTransmissionReflection = 1 << 3 | 1 << 1 | 1 << 0,
    SpecularReflection = 1 << 4 | 1 << 0,
    SpecularTransmission = 1 << 4 | 1 << 1,
    All = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

lazy_static! {
    static ref reversed_bxdf_flag_map: HashMap<isize, BxDFFlags> = {
        let mut reversed_map = HashMap::default();

        for flag in BxDFFlags::iter() {
            reversed_map.insert(flag as isize, flag);
        }

        reversed_map
    };
}

impl From<isize> for BxDFFlags {
    fn from(value: isize) -> Self {
        return match reversed_bxdf_flag_map.get(&value) {
            None => {
                panic!("illegal value: `{}`", value);
            }
            Some(flag) => *flag,
        };
    }
}

impl BitOr for BxDFFlags {
    type Output = isize;

    fn bitor(self, rhs: Self) -> Self::Output {
        return (self as isize) | (rhs as isize);
    }
}

impl BitAnd for BxDFFlags {
    type Output = isize;

    fn bitand(self, rhs: Self) -> Self::Output {
        return (self as isize) & (rhs as isize);
    }
}

impl BxDFFlags {
    pub fn is_diffuse(&self) -> bool {
        return (*self & BxDFFlags::Diffuse) > 0;
    }

    pub fn is_glossy(&self) -> bool {
        return (*self & BxDFFlags::Glossy) > 0;
    }

    pub fn is_non_specular(&self) -> bool {
        let f = *self as isize;
        return f & (BxDFFlags::Diffuse as isize | BxDFFlags::Glossy as isize) > 0;
    }

    pub fn is_reflective(&self) -> bool {
        return (*self & BxDFFlags::Reflection) > 0;
    }

    pub fn is_specular(&self) -> bool {
        return (*self & BxDFFlags::Specular) > 0;
    }

    pub fn is_transmissive(&self) -> bool {
        return (*self & BxDFFlags::Transmission) > 0;
    }
}

pub struct BSDFSample {
    pub f: SampledSpectrum,
    pub wi: Vector3f,
    pub pdf: f64,
    pub flags: BxDFFlags,
    pub eta: f64,
    // default args: eta = 1.0
    pub pdf_is_proportional: bool,
    // default args: pdfIsProportional = false
}

impl BSDFSample {
    pub fn is_reflection(&self) -> bool {
        return self.flags.is_reflective();
    }

    pub fn is_specular(&self) -> bool {
        return self.flags.is_specular();
    }

    pub fn is_transmission(&self) -> bool {
        return self.flags.is_transmissive();
    }
}

pub trait BxDF {
    fn fork(&self) -> Arc<dyn BxDF>;

    fn flags(&self) -> BxDFFlags;

    fn f(&self, wo: Vector3f, wi: Vector3f, mode: TransportMode) -> SampledSpectrum;

    fn sample_f(
        &self,
        wo: Vector3f,
        uc: f64,
        u: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> Option<BSDFSample>;

    fn pdf(
        &self,
        wo: Vector3f,
        wi: Vector3f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
    ) -> f64;
}
