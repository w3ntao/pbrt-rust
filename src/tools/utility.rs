use crate::core::pbrt::*;

pub fn load_dragon(material: Arc<dyn Material>) -> BVH {
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
        let primitive = GeometricPrimitive::new(t, material.clone());
        dragon_model.add(Arc::new(primitive));
    }
    dragon_model.build_index();

    return dragon_model;
}
