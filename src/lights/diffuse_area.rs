use crate::pbrt::*;

pub struct DiffuseAreaLight {
    /*
    Shape shape;
    FloatTexture alpha;
    f64 area;
    bool twoSided;
    const DenselySampledSpectrum *Lemit;
    f64 scale;
    Image image;
    const RGBColorSpace *imageColorSpace;
    */
    base: LightBase,
    shape: Arc<dyn Shape>,
    area: f64,
    two_sided: bool,
    lemit: DenselySampledSpectrum,
    scale: f64,
    image: Option<Image>,
    image_color_space: Arc<RGBColorSpace>,
}
