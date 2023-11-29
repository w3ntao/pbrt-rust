use crate::pbrt::*;

pub struct SpectrumImageTexture {
    image_texture_base: ImageTextureBase,
    // TODO: SpectrumType is currently missing
    spectrum_type: SpectrumType,
}

impl SpectrumImageTexture {
    pub fn new(
        render_from_texture: &Transform,
        parameters: &ParameterDict,
        spectrum_type: SpectrumType,
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

        return Self {
            image_texture_base,
            spectrum_type,
        };
    }
}

impl SpectrumTexture for SpectrumImageTexture {
    fn evaluate(&self, ctx: &TextureEvalContext, lambda: &SampledWavelengths) -> SampledSpectrum {
        let mut c = self.image_texture_base.mapping.map(ctx);
        c.st.y = 1.0 - c.st.y;

        // Lookup filtered RGB value in _MIPMap_
        let rgb = {
            let _rgb = self.image_texture_base.scale
                * self.image_texture_base.mipmap.filter(
                    c.st,
                    Vector2f::new(c.dsdx, c.dtdx),
                    Vector2f::new(c.dsdy, c.dtdy),
                );

            if self.image_texture_base.invert {
                RGB::new(1.0, 1.0, 1.0) - _rgb
            } else {
                _rgb
            }
            .clamp(0.0, Float::INFINITY)
        };

        let cs = self.image_texture_base.mipmap.color_space.as_ref();

        match self.spectrum_type {
            SpectrumType::Albedo => {
                return RGBAlbedoSpectrum::new(rgb.clamp(0.0, 1.0), cs).sample(lambda);
            }

            SpectrumType::Illuminant => {
                panic!("not implemented");
            }
            SpectrumType::Unbounded => {
                panic!("not implemented");
            }
        };
    }
}
