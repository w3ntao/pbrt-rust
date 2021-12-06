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
    let camera = PerspectiveCamera::new(
        Point::new(0.0, 0.0, 20.0),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 4.0,
        std::f32::consts::PI / 3.0);

    let triangle_0 = Triangle::new(
        Point::new(-2.0, 3.7, 0.0),
        Point::new(1.0, 2.0, 1.0),
        Point::new(3.0, 2.8, -2.0));
    let triangle_1 = Triangle::new(
        Point::new(3.0, 2.0, 3.0),
        Point::new(3.0, 2.0, -3.0),
        Point::new(-3.0, 2.0, -3.0));

    let sphere_0 = Sphere::new(Point::new(-2.0, 1.7, 0.0), 2.0);
    let sphere_1 = Sphere::new(Point::new(1.0, -1.0, 1.0), 2.2);
    let sphere_2 = Sphere::new(Point::new(3.0, 0.8, -2.0), 2.0);

    let quad = Quad::new(Point::new(1.0, -0.9, 4.5), Vector::new(-2.0, 0.0, 0.0), Vector::new(0.0, 0.1, -2.0));
    let aabox = AxisAlignedBox::new(Point::new(2.0, 1.5, -0.5), Point::new(3.0, 2.5, 2.5));

    let mut scene = BVH::default();
    scene.add(&triangle_0);
    scene.add(&triangle_1);

    scene.add(&sphere_0);
    scene.add(&sphere_1);
    scene.add(&sphere_2);

    scene.add(&quad);
    scene.add(&aabox);
    scene.build_index();

    let scene = scene;

    let world = World::new(&scene);
    let integrator = RayCastingIntegrator::new(&world);
    let renderer = Renderer::new(&camera, &integrator);
    let image = renderer.render(1920, 1440);
    image.write("out.ppm");

    return;
    // obj file loading
    let obj_file = "dragon.obj";

    let (models, materials) =
        tobj::load_obj(
            &obj_file,
            &tobj::LoadOptions::default(),
        )
            .expect("Failed to OBJ load file");

    // Note: If you don't mind missing the materials, you can generate a default.
    let materials = materials.expect("Failed to load MTL file");

    println!("Number of models          = {}", models.len());
    println!("Number of materials       = {}", materials.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("");
        println!("model[{}].name             = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!(
            "model[{}].face_count       = {}",
            i,
            mesh.face_arities.len()
        );

        let mut next_face = 0;
        for face in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[face] as usize;

            let face_indices = &mesh.indices[next_face..end];
            println!(" face[{}].indices          = {:?}", face, face_indices);

            if !mesh.texcoord_indices.is_empty() {
                let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
                println!(
                    " face[{}].texcoord_indices = {:?}",
                    face, texcoord_face_indices
                );
            }
            if !mesh.normal_indices.is_empty() {
                let normal_face_indices = &mesh.normal_indices[next_face..end];
                println!(
                    " face[{}].normal_indices   = {:?}",
                    face, normal_face_indices
                );
            }

            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in
        // this example.
        println!(
            "model[{}].positions        = {}",
            i,
            mesh.positions.len() / 3
        );
        assert!(mesh.positions.len() % 3 == 0);

        return;

        for vtx in 0..mesh.positions.len() / 3 {
            println!(
                "              position[{}] = ({}, {}, {})",
                vtx,
                mesh.positions[3 * vtx],
                mesh.positions[3 * vtx + 1],
                mesh.positions[3 * vtx + 2]
            );
        }
    }

    for (i, m) in materials.iter().enumerate() {
        println!("material[{}].name = \'{}\'", i, m.name);
        println!(
            "    material.Ka = ({}, {}, {})",
            m.ambient[0], m.ambient[1], m.ambient[2]
        );
        println!(
            "    material.Kd = ({}, {}, {})",
            m.diffuse[0], m.diffuse[1], m.diffuse[2]
        );
        println!(
            "    material.Ks = ({}, {}, {})",
            m.specular[0], m.specular[1], m.specular[2]
        );
        println!("    material.Ns = {}", m.shininess);
        println!("    material.d = {}", m.dissolve);
        println!("    material.map_Ka = {}", m.ambient_texture);
        println!("    material.map_Kd = {}", m.diffuse_texture);
        println!("    material.map_Ks = {}", m.specular_texture);
        println!("    material.map_Ns = {}", m.shininess_texture);
        println!("    material.map_Bump = {}", m.normal_texture);
        println!("    material.map_d = {}", m.dissolve_texture);

        for (k, v) in &m.unknown_param {
            println!("    material.{} = {}", k, v);
        }
    }
}
