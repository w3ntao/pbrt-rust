use crate::pbrt::*;

pub trait Film: Send + Sync {
    fn get_resolution(&self) -> Point2i;

    fn get_filter(&self) -> Arc<dyn Filter>;

    fn add_sample(&mut self, point_film: Point2i, spectrum: RGBColor);

    fn export_image(&self);
}
