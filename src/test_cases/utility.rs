use crate::fundamental::obj_loader::obj_to_triangles;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;

pub fn load_dragon() -> BVH {
    let triangles = obj_to_triangles("models/dragon.obj");
    let mut dragon_model = BVH::default();
    for t in triangles {
        dragon_model.add(t);
    }
    dragon_model.build_index();

    return dragon_model;
}
