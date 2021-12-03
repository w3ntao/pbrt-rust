use crate::ray_tracing::group::Group;

pub struct World<'a> {
    pub scene: &'a dyn Group<'a>,
}

impl<'a> World<'a> {
    pub fn new(_scene: &'a dyn Group<'a>) -> Self {
        return Self {
            scene: _scene,
        };
    }
}
