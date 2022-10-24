use crate::core::pbrt::*;

pub fn obj_to_triangles(obj_file: &str) -> Vec<Arc<NewTriangle>> {
    let (loaded_models, _materials) = tobj::load_obj(
        &obj_file,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )
    .expect("Failed to OBJ load file");

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
            mesh.positions[index + 2],
        ));
    }
    let vertices = vertices;

    let mut indices = vec![];
    for x in &mesh.indices {
        indices.push(*x as usize);
    }

    let triangle_mesh = TriangleMesh::new(vertices, indices);
    return triangle_mesh.build_triangle();
}
