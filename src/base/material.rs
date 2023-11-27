use crate::pbrt::*;

pub struct MaterialEvalContext {
    pub texture_eval_context: TextureEvalContext,
    pub wo: Vector3f,
    pub ns: Normal3f,
    pub dpdus: Vector3f,
}

pub trait Material {
    fn get_bsdf(&self, context: &MaterialEvalContext, lambda: &SampledWavelengths) -> Option<BSDF>;
    // in PBRT-minus get_bsdf works like get_bxdf in PBRT-v4
}

pub fn create_material(material_type: &str, parameter_dict: &ParameterDict) -> Arc<dyn Material> {
    return match material_type {
        "diffuse" => {
            let reflectance = parameter_dict.get_texture("reflectance");

            Arc::new(DiffuseMaterial::new(reflectance))
        }
        _ => {
            panic!("unknown material type: `{}`", material_type);
        }
    };
}
