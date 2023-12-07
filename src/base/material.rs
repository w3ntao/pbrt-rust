use crate::pbrt::*;

pub struct MaterialEvalContext {
    pub texture_eval_context: TextureEvalContext,
    pub wo: Vector3f,
    pub ns: Normal3f,
    pub dpdus: Vector3f,
}

impl Display for MaterialEvalContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaterialEvalContext [texture_eval_context {}  wo {}  ns {}  dpdus {}]",
            self.texture_eval_context, self.wo, self.ns, self.dpdus
        )
    }
}

impl MaterialEvalContext {
    pub fn new(si: &SurfaceInteraction) -> Self {
        return Self {
            texture_eval_context: TextureEvalContext::new(si),
            wo: si.interaction.wo,
            ns: si.shading.n,
            dpdus: si.shading.dpdu,
        };
    }
}

pub trait Material: Send + Sync {
    fn is_mix_material(&self) -> bool {
        return false;
    }

    fn get_bsdf(&self, context: &MaterialEvalContext, lambda: &SampledWavelengths) -> BSDF;
    // in PBRT-minus get_bsdf works like get_bxdf in PBRT-v4
}

pub fn create_material(material_type: &str, parameter_dict: &ParameterDict) -> Arc<dyn Material> {
    return match material_type {
        "diffuse" => {
            let key = "reflectance";
            let reflectance = if parameter_dict.has_texture(key) {
                parameter_dict.get_texture(key)
            } else {
                Arc::new(SpectrumConstantTexture::new(Arc::new(ConstSpectrum::new(
                    0.5,
                ))))
            };

            Arc::new(DiffuseMaterial::new(reflectance))
        }
        _ => {
            panic!("unknown material type: `{}`", material_type);
        }
    };
}
