use crate::pbrt::*;

pub struct DiffuseAreaLight {
    /*
    Shape shape;
    FloatTexture alpha;
    Float area;
    bool twoSided;
    const DenselySampledSpectrum *Lemit;
    Float scale;
    Image image;
    const RGBColorSpace *imageColorSpace;
    */
    base: LightBase,
    shape: Arc<dyn Shape>,
    area: Float,
    two_sided: bool,
    lemit: DenselySampledSpectrum,
    scale: Float,
    image: Option<Image>,
    image_color_space: Arc<RGBColorSpace>,
}
