use crate::pbrt::*;

pub struct TextureEvalContext {
    pub p: Point3f,
    pub dpdx: Vector3f,
    pub dpdy: Vector3f,
    pub n: Normal3f,
    pub uv: Point2f,
    pub dudx: Float,
    pub dudy: Float,
    pub dvdx: Float,
    pub dvdy: Float,
    pub face_index: usize,
}

pub struct ImageTextureBase {
    mapping: Arc<dyn TextureMapping2D>,
    filename: String,
    scale: Float,
    invert: bool,
    mipmap: MIPMap,
}

impl ImageTextureBase {
    pub fn new(
        mapping: Arc<dyn TextureMapping2D>,
        filename: &str,
        filter_options: MIPMapFilterOptions,
        wrap_mode: WrapMode,
        scale: Float,
        invert: bool,
        global_variable: &GlobalVariable,
    ) -> Self {
        let mipmap = MIPMap::create_from_file(filename, filter_options, wrap_mode, global_variable);

        return Self {
            mapping,
            filename: filename.to_string(),
            scale,
            invert,
            mipmap,
        };
    }
}

pub trait FloatTexture {
    fn evaluate(&self, ctx: &TextureEvalContext) -> Float;
}

pub trait SpectrumTexture {
    fn evaluate(&self, ctx: &TextureEvalContext, lambda: &SampledWavelengths) -> SampledSpectrum;
}

pub fn create_spectrum_texture(
    texture_type: &str,
    render_from_texture: &Transform,
    parameters: &ParameterDict,
    named_texture: &HashMap<String, Arc<dyn SpectrumTexture>>,
    global_variable: &GlobalVariable,
) -> Arc<dyn SpectrumTexture> {
    return match texture_type {
        "imagemap" => Arc::new(SpectrumImageTexture::new(
            render_from_texture,
            parameters,
            global_variable,
        )),
        "scale" => {
            let texture_id = parameters.get_string("tex", None);

            let spectrum_texture = match named_texture.get(&texture_id) {
                None => {
                    panic!("texture `{}` not found", texture_id);
                }
                Some(_texture) => _texture.clone(),
            };

            let scale = parameters.get_one_float("scale", Some(1.0));

            Arc::new(SpectrumScaledTexture::new(spectrum_texture, scale))
        }
        _ => {
            panic!("unknown SpectrumTexture type: `{}`", texture_type);
        }
    };
}
