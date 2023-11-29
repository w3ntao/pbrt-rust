use crate::pbrt::*;
use crate::textures::uv_mapping::UVMapping;

pub struct TexCoord2D {
    pub st: Point2f,
    pub dsdx: Float,
    pub dsdy: Float,
    pub dtdx: Float,
    pub dtdy: Float,
}

pub trait TextureMapping2D: Send + Sync {
    fn map(&self, ctx: &TextureEvalContext) -> TexCoord2D;
}

pub fn create_texture_mapping_2d(
    render_from_texture: &Transform,
    parameters: &ParameterDict,
) -> Arc<dyn TextureMapping2D> {
    let mapping_type = parameters.get_string("mapping", Some("uv".to_string()));

    return match mapping_type.as_str() {
        "uv" => {
            let su = parameters.get_one_float("uscale", Some(1.0));
            let sv = parameters.get_one_float("vscale", Some(1.0));
            let du = parameters.get_one_float("udelta", Some(0.0));
            let dv = parameters.get_one_float("vdelta", Some(0.0));

            Arc::new(UVMapping::new(su, sv, du, dv))
        }
        _ => {
            panic!("2D texture mapping `{}` not implemented", mapping_type);
        }
    };
}
