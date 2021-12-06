use crate::fundamental::point::Point;
use crate::fundamental::vector::Vector;

use crate::ray_tracing::primitives::triangle::Triangle;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::primitives::axis_aligned_box::AxisAlignedBox;
use crate::ray_tracing::primitives::quad::Quad;

use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::simple_group::SimpleGroup;
use crate::ray_tracing::groups::bvh::BVH;

use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::integrators::ray_casting::RayCastingIntegrator;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    // obj file loading
    let obj_file = "models/dragon.obj";

    let (loaded_models, materials) =
        tobj::load_obj(
            &obj_file,
            &tobj::LoadOptions {
                //single_index: true,
                triangulate: true,
                ..Default::default()
            },
        )
            .expect("Failed to OBJ load file");

    if loaded_models.len() != 1 {
        panic!("Currently we deal with only 1 model per time")
    }

    let mut scene = BVH::default();
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

        triangles.push(Triangle::new(
            vertices[p0_idx as usize],
            vertices[p1_idx as usize],
            vertices[p2_idx as usize],
        ));
    }
    for t in &triangles {
        scene.add(t);
    }
    scene.build_index();

    let camera = PerspectiveCamera::new(
        Point::new(-2.2, 0.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0);

    let world = World::new(&scene);
    let integrator = RayCastingIntegrator::new(&world);
    let renderer = Renderer::new(&camera, &integrator);
    let image = renderer.render(2000, 1500);
    image.write("dragon.ppm");
}
