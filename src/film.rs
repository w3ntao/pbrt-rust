use crate::pbrt::*;

pub struct SimpleRGBFilm {
    pub resolution: Point2i,
    pub filename: String,
    pub filter: BoxFilter,
}
