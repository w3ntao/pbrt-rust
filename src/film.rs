use crate::pbrt::*;

#[derive(Clone)]
pub struct SimpleRGBFilm {
    pub resolution: Point2i,
    pub filename: String,
    pub filter: BoxFilter,
}

impl SimpleRGBFilm {
    pub fn new(_resolution: Point2i, _filename: &String, _filter: BoxFilter) -> Self {
        return SimpleRGBFilm {
            resolution: _resolution,
            filename: _filename.clone(),
            filter: _filter,
        };
    }
}
