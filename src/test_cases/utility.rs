use crate::fundamental::obj_loader::obj_to_triangles;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use std::path::Path;
use std::process;

pub fn get_file_name(full_path: &str) -> String {
    let file_name_with_postfix = Path::new(full_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();
    let file_name = &file_name_with_postfix[0..(&file_name_with_postfix).len() - 3];

    return file_name.to_string();
}

pub fn load_dragon() -> BVH {
    let dragon_path = "models/dragon.obj";

    if !Path::new(dragon_path).exists() {
        println!("\nError: couldn't find model: dragon");
        println!("Please download dragon model from https://casual-effects.com:");
        println!("$ wget 'https://casual-effects.com/g3d/data10/research/model/dragon/dragon.zip'");
        println!("$ mkdir models");
        println!("$ unzip dragon.zip -d models/");
        process::exit(1);
    }

    let triangles = obj_to_triangles(dragon_path);
    let mut dragon_model = BVH::default();
    for t in triangles {
        dragon_model.add(t);
    }
    dragon_model.build_index();

    return dragon_model;
}
