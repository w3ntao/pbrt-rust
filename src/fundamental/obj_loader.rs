use std::rc::Rc;
use crate::fundamental::point::Point;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::triangle::Triangle;

pub fn obj_to_triangles(obj_file: &str) -> Vec<Rc<Triangle>> {
    let (loaded_models, materials) =
        tobj::load_obj(
            &obj_file,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        ).expect("Failed to OBJ load file");

    if loaded_models.len() != 1 {
        panic!("Currently we deal with only 1 model per time")
    }

    let model = &loaded_models[0];
    let mesh = &model.mesh;

    let mut vertices = Vec::new();
    for index in (0..mesh.positions.len()).step_by(3) {
        vertices.push(Point::new(
            mesh.positions[index],
            mesh.positions[index + 1],
            mesh.positions[index + 2]));
    }
    let vertices = vertices;

    let mut triangles = Vec::new();
    for index in (0..mesh.indices.len()).step_by(3) {
        let p0_idx = mesh.indices[index];
        let p1_idx = mesh.indices[index + 1];
        let p2_idx = mesh.indices[index + 2];

        triangles.push(Rc::new(Triangle::new(
            vertices[p0_idx as usize],
            vertices[p1_idx as usize],
            vertices[p2_idx as usize],
        )));
    }

    return triangles;
}
