use crate::pbrt::*;

pub struct SpectrumImageTexture {
    image_texture_base: ImageTextureBase,
    // TODO: SpectrumType is currently missing
}

impl SpectrumImageTexture {
    pub fn new(
        render_from_texture: &Transform,
        parameters: &ParameterDict,
        global_variable: &GlobalVariable,
    ) -> Self {
        // TODO: implementing SpectrumImageTexture::Create() (pbrt-v4/src/pbrt/textures.cpp)
        let map = create_texture_mapping_2d(render_from_texture, parameters);

        let max_anisotropy = parameters.get_one_float("maxanisotropy", Some(8.0));
        let filter = parameters.get_string("filter", Some("bilinear".to_string()));

        let filter_options = MIPMapFilterOptions {
            filter: parse_filter_function(&filter),
            max_anisotropy,
        };

        let wrap_string = parameters.get_string("wrap", Some("repeat".to_string()));
        let wrap_mode = parse_wrap_mode(&wrap_string);

        let scale = parameters.get_one_float("scale", Some(1.0));
        let invert = parameters.get_one_bool("invert", Some(false));

        let filename = parameters.get_string("filename", None);

        let image_texture_base = ImageTextureBase::new(
            map,
            &filename,
            filter_options,
            wrap_mode,
            scale,
            invert,
            global_variable,
        );

        return Self { image_texture_base };
    }
}

impl SpectrumTexture for SpectrumImageTexture {
    fn evaluate(&self, ctx: &TextureEvalContext, lambda: &SampledWavelengths) -> SampledSpectrum {
        unreachable!();
    }
}
