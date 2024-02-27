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

    fn get_bsdf(&self, ctx: &MaterialEvalContext, lambda: &mut SampledWavelengths) -> BSDF;
    // in pbrt-rust get_bsdf works like get_bxdf in PBRT-v4
}

pub fn create_material(material_type: &str, parameter_dict: &ParameterDict) -> Arc<dyn Material> {
    return match material_type {
        "diffuse" => {
            let reflectance = {
                let key = "reflectance";
                if parameter_dict.has_texture(key) {
                    parameter_dict.get_texture(key)
                } else if parameter_dict.has_rgb(key) {
                    let rgb_color = parameter_dict.get_rgb(key, None);
                    let spectrum = RGBAlbedoSpectrum::new(rgb_color);

                    Arc::new(SpectrumConstantTexture::new(Arc::new(spectrum)))
                } else {
                    panic!("couldn't find `{}` in parameters", key);
                }
            };

            Arc::new(DiffuseMaterial::new(reflectance))
        }

        "coateddiffuse" => {
            let reflectance = {
                let key = "reflectance";
                if parameter_dict.has_texture(key) {
                    parameter_dict.get_texture(key)
                } else if parameter_dict.has_rgb(key) {
                    let rgb_color = parameter_dict.get_rgb(key, None);
                    let spectrum = RGBAlbedoSpectrum::new(rgb_color);
                    Arc::new(SpectrumConstantTexture::new(Arc::new(spectrum)))
                } else {
                    let spectrum = RGBAlbedoSpectrum::new(RGB::new(0.5, 0.5, 0.5));
                    Arc::new(SpectrumConstantTexture::new(Arc::new(spectrum)))
                }
            };

            let u_roughness = {
                let roughness = parameter_dict.get_one_float("uroughness", Some(0.0));
                Arc::new(FloatConstantTexture::new(roughness))
            };

            let v_roughness = {
                let roughness = parameter_dict.get_one_float("vroughness", Some(0.0));
                Arc::new(FloatConstantTexture::new(roughness))
            };

            let thickness = {
                let thickness = parameter_dict.get_one_float("vroughness", Some(0.01));
                Arc::new(FloatConstantTexture::new(thickness))
            };

            let eta = {
                let eta = parameter_dict.get_one_float("eta", Some(1.5));
                Arc::new(ConstSpectrum::new(eta))
            };

            let max_depth = parameter_dict.get_one_integer("maxdepth", Some(10));
            let n_samples = parameter_dict.get_one_integer("nsample", Some(1));

            let g = {
                let g = parameter_dict.get_one_float("g", Some(0.0));
                Arc::new(FloatConstantTexture::new(g))
            };

            let albedo = {
                let albedo = parameter_dict.get_one_float("albedo", Some(0.0));
                Arc::new(SpectrumConstantTexture::new(Arc::new(ConstSpectrum::new(
                    albedo,
                ))))
            };

            let remap_roughness = parameter_dict.get_one_bool("remaproughness", Some(true));

            let coated_diffuse = CoatedDiffuseMaterial::new(
                reflectance,
                albedo,
                u_roughness,
                v_roughness,
                thickness,
                g,
                eta,
                remap_roughness,
                max_depth as usize,
                n_samples as usize,
            );

            Arc::new(coated_diffuse)
        }

        _ => {
            panic!("unknown material type: `{}`", material_type);
        }
    };
}
