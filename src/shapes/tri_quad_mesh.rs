extern crate ply_rs;
use crate::pbrt::*;
use flate2::bufread::GzDecoder;
use ply_rs::ply;

pub struct TriQuadMesh {
    pub p: Vec<Point3f>,
    pub n: Vec<Normal3f>,
    pub uv: Vec<Point2f>,
    pub tri_indices: Vec<usize>,
}

pub fn read_ply(ply_file_path: &str) -> TriQuadMesh {
    let ply_model = {
        let ply_parser = ply_rs::parser::Parser::<ply::DefaultElement>::new();

        let _file = &mut match File::open(ply_file_path) {
            Ok(opened_file) => opened_file,
            Err(_) => {
                panic!("couldn't read PLY from `{}`", ply_file_path);
            }
        };

        let optional_model = if ply_file_path.ends_with(".ply.gz") {
            let buffer_reader = BufReader::new(_file);
            let mut gzip_reader = GzDecoder::new(buffer_reader);
            ply_parser.read_ply(&mut gzip_reader)
        } else if ply_file_path.ends_with(".ply") {
            ply_parser.read_ply(_file)
        } else {
            panic!("unknown file format: {}", ply_file_path);
        };

        match optional_model {
            Ok(_model) => _model,
            Err(msg) => {
                panic!("fail to read PLY file `{}`:\n{}", ply_file_path, msg);
            }
        }
    };

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
                                    pnt.x = x as Float;
                                }
                            }
                            "y" => {
                                if let ply::Property::Float(y) = list2 {
                                    pnt.y = y as Float;
                                }
                            }
                            "z" => {
                                if let ply::Property::Float(z) = list2 {
                                    pnt.z = z as Float;
                                }
                            }
                            "nx" => {
                                has_normals = true;
                                if let ply::Property::Float(x) = list2 {
                                    nrm.x = x as Float;
                                }
                            }
                            "ny" => {
                                has_normals = true;
                                if let ply::Property::Float(y) = list2 {
                                    nrm.y = y as Float;
                                }
                            }
                            "nz" => {
                                has_normals = true;
                                if let ply::Property::Float(z) = list2 {
                                    nrm.z = z as Float;
                                }
                            }
                            "u" | "s" => {
                                has_uvs = true;
                                if let ply::Property::Float(x) = list2 {
                                    pt2.x = x as Float;
                                }
                            }
                            "v" | "t" => {
                                has_uvs = true;
                                if let ply::Property::Float(y) = list2 {
                                    pt2.y = y as Float;
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
                                    nrm.x = x as Float;
                                }
                            }
                            "ny" => {
                                has_normals = true;
                                if let ply::Property::Float(y) = list2 {
                                    nrm.y = y as Float;
                                }
                            }
                            "nz" => {
                                has_normals = true;
                                if let ply::Property::Float(z) = list2 {
                                    nrm.z = z as Float;
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
