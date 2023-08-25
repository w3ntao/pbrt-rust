use crate::pbrt::*;

pub struct TriQuadMesh {
    pub p: Vec<Point3f>,
    pub n: Vec<Normal3f>,
    pub uv: Vec<Point2f>,
    pub tri_indices: Vec<usize>,
}

pub fn read_ply(ply_file_path: &str) -> TriQuadMesh {
    // create a parser
    let ply_parser = ply_rs::parser::Parser::<ply::DefaultElement>::new();

    let path_without_gz = if ply_file_path.ends_with(".ply.gz") {
        // TODO: rewrite ply-rs to read gzipped files
        let without_gz = &ply_file_path[..(ply_file_path.len() - 3)];
        if !Path::new(without_gz).exists() {
            println!("please unzip `{}` with command:", ply_file_path);
            println!("$ gzip {} -cdk > {}", ply_file_path, without_gz);
            exit(1);
        }
        without_gz
    } else {
        &ply_file_path
    };

    println!("reading `{}`", path_without_gz);
    // use the parser: read the entire file
    let _ply_model = ply_parser.read_ply(&mut File::open(path_without_gz).unwrap());

    // make sure it did work
    if !_ply_model.is_ok() {
        panic!("illegal PLY format: {}", path_without_gz);
    }

    let ply_model = _ply_model.unwrap();

    let mut p: Vec<Point3f> = Vec::new();
    let mut n: Vec<Normal3f> = Vec::new();
    let mut uv: Vec<Point2f> = Vec::new();
    let mut has_normals: bool = false;
    let mut has_uvs: bool = false;
    let mut vertex_indices: Vec<usize> = Vec::new();

    for (name, list) in ply_model.payload.into_iter() {
        match name.as_ref() {
            "vertex" => {
                for elem in list.into_iter() {
                    let mut pnt: Point3f = Point3f::default();
                    let mut nrm: Normal3f = Normal3f::default();
                    let mut pt2: Point2f = Point2f::default();
                    for (name2, list2) in elem.into_iter() {
                        match name2.as_ref() {
                            "x" => {
                                if let ply::Property::Float(x) = list2 {
                                    pnt.x = x;
                                }
                            }
                            "y" => {
                                if let ply::Property::Float(y) = list2 {
                                    pnt.y = y;
                                }
                            }
                            "z" => {
                                if let ply::Property::Float(z) = list2 {
                                    pnt.z = z;
                                }
                            }
                            "nx" => {
                                has_normals = true;
                                if let ply::Property::Float(x) = list2 {
                                    nrm.x = x;
                                }
                            }
                            "ny" => {
                                has_normals = true;
                                if let ply::Property::Float(y) = list2 {
                                    nrm.y = y;
                                }
                            }
                            "nz" => {
                                has_normals = true;
                                if let ply::Property::Float(z) = list2 {
                                    nrm.z = z;
                                }
                            }
                            "u" | "s" => {
                                has_uvs = true;
                                if let ply::Property::Float(x) = list2 {
                                    pt2.x = x;
                                }
                            }
                            "v" | "t" => {
                                has_uvs = true;
                                if let ply::Property::Float(y) = list2 {
                                    pt2.y = y;
                                }
                            }
                            _ => {
                                println!("name2 = {:?}", name2);
                                unreachable!();
                            }
                        }
                    }
                    p.push(pnt);
                    if has_normals {
                        n.push(nrm);
                    }
                    if has_uvs {
                        uv.push(pt2);
                    }
                }
            }
            "face" => {
                for elem in list.into_iter() {
                    let mut nrm: Normal3f = Normal3f::default();
                    for (name2, list2) in elem.into_iter() {
                        match name2.as_ref() {
                            "vertex_indices" => {
                                if let ply::Property::ListInt(li) = list2 {
                                    let mut _v_indices: Vec<usize> = Vec::new();
                                    for i in li.into_iter() {
                                        _v_indices.push(i as usize);
                                    }
                                    // println!("vertex_indices = {:?}", vertex_indices);
                                    if _v_indices.len() != 3 {
                                        if _v_indices.len() == 4 {
                                            // handle quads (split it into 2 triangles)
                                            let v1 = _v_indices[0];
                                            let v3 = _v_indices[2];
                                            let v4 = _v_indices.pop().unwrap();
                                            _v_indices.push(v4);
                                            _v_indices.push(v1);
                                            _v_indices.push(v3);
                                        } else {
                                            panic!("plymesh: Ignoring face with {} vertices (only triangles and quads are supported!)",
                                                   _v_indices.len());
                                        }
                                    }
                                    // now we can add the indices to the triangle mesh vertex indices
                                    for vi in _v_indices {
                                        vertex_indices.push(vi.try_into().unwrap());
                                    }
                                } else if let ply::Property::ListUInt(li) = list2 {
                                    let mut _v_indices: Vec<usize> = Vec::new();
                                    for i in li.into_iter() {
                                        _v_indices.push(i as usize);
                                    }
                                    // println!("vertex_indices = {:?}", vertex_indices);
                                    if _v_indices.len() != 3 {
                                        if _v_indices.len() == 4 {
                                            // handle quads (split it into 2 triangles)
                                            let v1 = _v_indices[0];
                                            let v3 = _v_indices[2];
                                            let v4 = _v_indices.pop().unwrap();
                                            _v_indices.push(v4);
                                            _v_indices.push(v1);
                                            _v_indices.push(v3);
                                        } else {
                                            panic!("plymesh: Ignoring face with {} vertices (only triangles and quads are supported!)",
                                                   _v_indices.len());
                                        }
                                    }
                                    // now we can add the indices to the triangle mesh vertex indices
                                    for vi in _v_indices {
                                        vertex_indices.push(vi.try_into().unwrap());
                                    }
                                }
                            }
                            "nx" => {
                                has_normals = true;
                                if let ply::Property::Float(x) = list2 {
                                    nrm.x = x;
                                }
                            }
                            "ny" => {
                                has_normals = true;
                                if let ply::Property::Float(y) = list2 {
                                    nrm.y = y;
                                }
                            }
                            "nz" => {
                                has_normals = true;
                                if let ply::Property::Float(z) = list2 {
                                    nrm.z = z;
                                }
                            }
                            _ => panic!("couldn't parse {}", name2),
                        }
                    }
                }
            }
            _ => panic!("couldn't parse `{}`", name),
        }
    }

    return TriQuadMesh {
        p,
        n,
        uv,
        tri_indices: vertex_indices,
    };
}
