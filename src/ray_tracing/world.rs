use crate::ray_tracing::groups::group_trait::Group;

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
