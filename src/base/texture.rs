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
}

impl Display for TextureEvalContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "TextureEvalContext [ p {}  dpdx {}  dpdy {}  n {}  uv {}  dudx {} dudy {}  dvdx {}  dvdy {}]",
               self.p, self.dpdx, self.dpdy,self.n, self.uv, self.dudx, self.dudy, self.dvdx, self.dvdy)
    }
}

impl TextureEvalContext {
    pub fn new(si: &SurfaceInteraction) -> Self {
        return Self {
            p: Point3f::from(si.pi),
            dpdx: si.dpdx,
            dpdy: si.dpdy,
            n: si.n,
            uv: si.uv,
            dudx: 0.0,
            dudy: 0.0,
            dvdx: 0.0,
            dvdy: 0.0,
        };
    }
}

pub struct ImageTextureBase {
    pub mapping: Arc<dyn TextureMapping2D>,
    pub filename: String,
    pub scale: Float,
    pub invert: bool,
    pub mipmap: MIPMap,
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

pub trait SpectrumTexture: Send + Sync {
    fn evaluate(&self, ctx: &TextureEvalContext, lambda: &SampledWavelengths) -> SampledSpectrum;
}

pub fn create_spectrum_texture(
    texture_type: &str,
    render_from_texture: &Transform,
    parameters: &ParameterDict,
    spectrum_type: SpectrumType,
    global_variable: &GlobalVariable,
) -> Arc<dyn SpectrumTexture> {
    return match texture_type {
        "imagemap" => Arc::new(SpectrumImageTexture::new(
            render_from_texture,
            parameters,
            spectrum_type,
            global_variable,
        )),
        "scale" => {
            let spectrum_texture = parameters.get_texture("tex");

            let scale = parameters.get_one_float("scale", Some(1.0));

            Arc::new(SpectrumScaledTexture::new(spectrum_texture, scale))
        }
        _ => {
            panic!("unknown SpectrumTexture type: `{}`", texture_type);
        }
    };
}
