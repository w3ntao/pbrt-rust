use crate::GroupTrait;

pub struct World<'a> {
    pub scene: &'a (dyn GroupTrait<'a> + 'a),
}

impl<'a> World<'a> {
    pub fn new(_scene: &'a (dyn GroupTrait<'a> + 'a)) -> Self {
        return Self {
            scene: _scene,
        };
    }
}
