use crate::ray_tracing::group::group_trait::GroupTrait;

pub struct World<'a> {
    pub scene: &'a dyn GroupTrait<'a>,
}

impl<'a> World<'a> {
    pub fn new(_scene: &'a dyn GroupTrait<'a>) -> Self {
        return Self {
            scene: _scene,
        };
    }
}
