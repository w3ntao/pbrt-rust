use crate::pbrt::*;

#[derive(Clone)]
pub struct SimpleRGBFilm {
    pub resolution: Point2i,
    pub filename: String,
    pub filter: Arc<BoxFilter>,
}

impl SimpleRGBFilm {
    pub fn new(_resolution: Point2i, _filename: &String, _filter: Arc<BoxFilter>) -> Self {
        return SimpleRGBFilm {
            resolution: _resolution,
            filename: _filename.clone(),
            filter: _filter.clone(),
        };
    }

    pub fn add_sample(&self) {
        panic!("implement me");
    }
}
