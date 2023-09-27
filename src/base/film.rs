use crate::pbrt::*;

pub trait Film: Send + Sync {
    fn get_resolution(&self) -> Point2i;

    fn get_filter(&self) -> Arc<dyn Filter>;

    fn get_pixel_rgb(&self, p: Point2i) -> RGB;

    fn sample_wavelengths(&self, u: Float) -> SampledWavelengths;

    fn add_sample(
        &mut self,
        point_film: Point2i,
        l: &SampledSpectrum,
        lambda: &SampledWavelengths,
        weight: Float,
    );

    fn export_image(&self);
}
