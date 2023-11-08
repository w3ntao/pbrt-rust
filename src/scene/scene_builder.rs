use crate::pbrt::*;

fn build_look_at_transform(pos: Point3f, look: Point3f, up: Vector3f) -> Transform {
    let mut world_from_camera = SquareMatrix::<4>::zero();
    world_from_camera[0][3] = pos.x;
    world_from_camera[1][3] = pos.y;
    world_from_camera[2][3] = pos.z;
    world_from_camera[3][3] = 1.0;

    let dir = (look - pos).normalize();
    if up.normalize().cross(dir).length() == 0.0 {
        panic!("LookAt: `up` vector and viewing direction are pointing in the same direction");
    }

    let right = up.normalize().cross(dir).normalize();
    let new_up = dir.cross(right);

    world_from_camera[0][0] = right.x;
    world_from_camera[1][0] = right.y;
    world_from_camera[2][0] = right.z;
    world_from_camera[3][0] = 0.0;
    world_from_camera[0][1] = new_up.x;
    world_from_camera[1][1] = new_up.y;
    world_from_camera[2][1] = new_up.z;
    world_from_camera[3][1] = 0.0;
    world_from_camera[0][2] = dir.x;
    world_from_camera[1][2] = dir.y;
    world_from_camera[2][2] = dir.z;
    world_from_camera[3][2] = 0.0;

    let camera_from_world = world_from_camera.inverse();
    return Transform::new_with_inverse(camera_from_world, world_from_camera);
}

fn parse_json(path: &str) -> Value {
    let mut file = match File::open(path) {
        Ok(_file) => _file,
        Err(_) => {
            panic!("fail to read `{}`", path);
        }
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("JSON was not well-formatted");
}

struct SceneEntity {
    pub initialized: bool,
    pub name: String,
    pub parameters: ParameterDict,
    pub camera_transform: CameraTransform,
}

impl Default for SceneEntity {
    fn default() -> Self {
        return Self {
            initialized: false,
            name: "".parse().unwrap(),
            parameters: ParameterDict::default(),
            camera_transform: CameraTransform::nan(),
        };
    }
}

fn build_film(
    film_entity: &SceneEntity,
    _filter: Arc<BoxFilter>,
    global_variable: &GlobalVariable,
) -> Arc<Mutex<dyn Film>> {
    let xresolution = film_entity.parameters.get_one_integer("xresolution", None);
    let yresolution = film_entity.parameters.get_one_integer("yresolution", None);

    let resolution = Point2i::new(xresolution, yresolution);
    let filename = film_entity.parameters.get_string("filename", None);

    match film_entity.name.as_str() {
        "rgb" => {
            let exposure_time = 1.0;
            let sensor =
                PixelSensor::create(&film_entity.parameters, exposure_time, global_variable);

            return Arc::new(Mutex::new(RGBFilm::new(
                resolution,
                &filename,
                Arc::new(sensor),
                _filter,
                global_variable,
            )));
        }
        _ => {
            panic!("film `{}` not implemented", film_entity.name);
        }
    };
}

fn build_camera(camera_entity: &SceneEntity, resolution: Point2i) -> Arc<dyn Camera> {
    return Arc::new(match camera_entity.name.as_str() {
        "perspective" => PerspectiveCamera::new(
            camera_entity.camera_transform,
            camera_entity.parameters.clone(),
            resolution,
        ),
        _ => {
            panic!("unknown camera type: `{}`", camera_entity.name);
        }
    });
}

#[derive(Copy, Clone)]
struct GraphicsState {
    current_transform: Transform,
    reverse_orientation: bool,
}

impl GraphicsState {
    pub fn new() -> Self {
        return GraphicsState {
            current_transform: Transform::identity(),
            reverse_orientation: false,
        };
    }
}

pub struct SceneBuilder {
    graphics_state: GraphicsState,
    pushed_graphics_state: Vec<GraphicsState>,

    named_coordinate_systems: HashMap<String, Transform>,
    named_texture: HashMap<String, Arc<dyn SpectrumTexture>>,

    render_from_world: Transform,
    primitives: Vec<Arc<dyn Primitive>>,

    film_entity: SceneEntity,
    camera_entity: SceneEntity,

    root: Option<String>,

    global_variable: Arc<GlobalVariable>,
}

impl SceneBuilder {
    pub fn new(global_variable: Arc<GlobalVariable>) -> Self {
        return SceneBuilder {
            graphics_state: GraphicsState::new(),
            pushed_graphics_state: Vec::new(),

            named_coordinate_systems: HashMap::new(),
            named_texture: HashMap::new(),

            render_from_world: Transform::identity(),
            primitives: vec![],

            film_entity: SceneEntity::default(),
            camera_entity: SceneEntity::default(),

            root: None,
            global_variable,
        };
    }

    fn get_filepath(&self, file_basename: &str) -> String {
        return match &self.root {
            None => file_basename.to_string(),
            Some(root) => format!("{}/{}", root, file_basename),
        };
    }

    fn render_from_object(&self) -> Transform {
        return self.render_from_world * self.graphics_state.current_transform;
    }

    fn parse_coord_sys_transform(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "CoordSysTransform");

        let name = json_value_to_string(tokens[1].clone());
        self.graphics_state.current_transform = match self.named_coordinate_systems.get(&name) {
            None => {
                panic!("couldn't find key {}", name);
            }
            Some(transform) => *transform,
        };
    }

    fn parse_look_at(&mut self, array: &Vec<Value>) {
        assert_eq!(json_value_to_string(array[0].clone()), "LookAt");

        let length = array.len();
        assert_eq!(length, 10);

        let mut data = [Float::NAN; 9];
        for idx in 1..length {
            let number_in_string = trim_quote(json_value_to_string(array[idx].clone()));

            data[idx - 1] = number_in_string.parse::<Float>().unwrap();
        }

        let position = Point3f::new(data[0], data[1], data[2]);
        let look = Point3f::new(data[3], data[4], data[5]);
        let up = Vector3f::new(data[6], data[7], data[8]);

        let transform_look_at = build_look_at_transform(position, look, up);

        self.graphics_state.current_transform =
            self.graphics_state.current_transform * transform_look_at;
    }

    fn parse_film(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "Film");

        self.film_entity.initialized = true;
        self.film_entity.name = json_value_to_string(tokens[1].clone());
        if self.film_entity.name != "rgb" {
            println!("warning: only `rgb` film is supported for the moment.");
            self.film_entity.name = "rgb".to_string();
        }

        self.film_entity.parameters = ParameterDict::build_parameter_dict(&tokens[2..], None);
    }

    fn parse_camera(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "Camera");

        let name = json_value_to_string(tokens[1].clone());
        let parameter_dict = ParameterDict::build_parameter_dict(&tokens[2..], self.root.clone());

        let camera_from_world = self.graphics_state.current_transform;
        let world_from_camera = camera_from_world.inverse();

        self.named_coordinate_systems
            .insert("camera".to_string(), camera_from_world.inverse());

        let camera_transform =
            CameraTransform::new(world_from_camera, RenderingCoordinateSystem::CameraWorld);

        self.render_from_world = camera_transform.render_from_world();

        self.camera_entity.initialized = true;
        self.camera_entity.name = name;
        self.camera_entity.camera_transform = camera_transform;
        self.camera_entity.parameters = parameter_dict;
    }

    fn parse_rotate(&mut self, tokens: &Vec<Value>) {
        assert_eq!(tokens.len(), 5);
        assert_eq!(json_value_to_string(tokens[0].clone()), "Rotate");

        let floats = json_values_to_floats(&tokens.clone()[1..]);
        assert_eq!(floats.len(), 4);

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::rotate(floats[0], floats[1], floats[2], floats[3]);
    }

    fn parse_scale(&mut self, tokens: &Vec<Value>) {
        assert_eq!(tokens.len(), 4);
        assert_eq!(json_value_to_string(tokens[0].clone()), "Scale");

        let floats = json_values_to_floats(&tokens.clone()[1..]);
        assert_eq!(floats.len(), 3);

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::scale(floats[0], floats[1], floats[2]);
    }

    fn parse_shape(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "Shape");

        let parameters = ParameterDict::build_parameter_dict(&tokens[2..], self.root.clone());

        let render_from_object = self.render_from_object();
        let object_from_render = render_from_object.inverse();

        let reverse_orientation = self.graphics_state.reverse_orientation;

        let name = json_value_to_string(tokens[1].clone());
        match name.as_str() {
            "disk" => {
                // TODO: disk not implemented
            }

            "loopsubdiv" => {
                let levels = parameters.get_one_integer("levels", Some(3)) as usize;
                let indices_i32 = parameters.get_integer_array("indices");
                let indices = indices_i32.into_iter().map(|x| x as usize).collect();

                let points = parameters.get_point3_array("P");

                let triangles = loop_subdivide(&render_from_object, levels, indices, points);

                for _triangle in &triangles {
                    let primitive = SimplePrimitive::new(_triangle.clone());
                    self.primitives.push(Arc::new(primitive));
                }
            }

            "sphere" => {
                let radius = parameters.get_one_float("radius", Some(1.0));
                let zmin = parameters.get_one_float("zmin", Some(-radius));
                let zmax = parameters.get_one_float("zmax", Some(radius));
                let phimax = parameters.get_one_float("phimax", Some(360.0));

                let sphere = Sphere::new(
                    render_from_object,
                    object_from_render,
                    reverse_orientation,
                    radius,
                    zmin,
                    zmax,
                    phimax,
                );

                let primitive = SimplePrimitive::new(Arc::new(sphere));
                self.primitives.push(Arc::new(primitive));
            }

            "trianglemesh" => {
                let indices = parameters.get_integer_array("indices");
                let points = parameters.get_point3_array("P");

                let normals = parameters.get_normal3_array("N");

                let mesh = TriangleMesh::new(
                    &render_from_object,
                    points,
                    indices.into_iter().map(|x| x as usize).collect(),
                    normals,
                );

                let triangles = mesh.create_triangles();
                for _triangle in &triangles {
                    let primitive = SimplePrimitive::new(_triangle.clone());
                    self.primitives.push(Arc::new(primitive));
                }
            }

            "plymesh" => {
                let file_path = &parameters.get_string("filename", None);
                let tri_quad_mesh = read_ply(file_path);

                if tri_quad_mesh.tri_indices.len() > 0 {
                    let triangle_mesh = TriangleMesh::new(
                        &render_from_object,
                        tri_quad_mesh.p,
                        tri_quad_mesh.tri_indices,
                        tri_quad_mesh.n,
                    );

                    let triangles = triangle_mesh.create_triangles();
                    for _triangle in &triangles {
                        let primitive = SimplePrimitive::new(_triangle.clone());
                        self.primitives.push(Arc::new(primitive));
                    }
                }
            }
            _ => {
                panic!("unknown Shape: `{}`", name);
            }
        };
    }

    fn parse_texture(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "Texture");

        let texture_name = json_value_to_string(tokens[1].clone());

        let color_type = json_value_to_string(tokens[2].clone());
        let texture_type = json_value_to_string(tokens[3].clone());

        match color_type.as_str() {
            "spectrum" => {
                let parameter_dict =
                    ParameterDict::build_parameter_dict(&tokens[4..], self.root.clone());

                //TODO: SpectrumType is missing in creating SpectrumTexture
                let texture = create_spectrum_texture(
                    &texture_type,
                    &self.render_from_object(),
                    &parameter_dict,
                    &self.named_texture,
                    self.global_variable.as_ref(),
                );

                self.named_texture.insert(texture_name, texture);
            }
            _ => {
                panic!(
                    "unknown color type and texture type: ({}, {})",
                    color_type, texture_type
                );
            }
        };
    }

    fn parse_transform(&mut self, tokens: &Vec<Value>) {
        assert_eq!(tokens.len(), 2);
        assert_eq!(json_value_to_string(tokens[0].clone()), "Transform");

        let value_list = json_value_to_floats(tokens[1].clone());
        assert_eq!(value_list.len(), 16);

        self.graphics_state.current_transform =
            Transform::new(SquareMatrix::<4>::from_vec(value_list)).transpose();
    }

    fn parse_translate(&mut self, tokens: &Vec<Value>) {
        assert_eq!(tokens.len(), 4);
        assert_eq!(json_value_to_string(tokens[0].clone()), "Translate");

        let floats = json_values_to_floats(&tokens.clone()[1..]);
        assert_eq!(floats.len(), 3);

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::translate(floats[0], floats[1], floats[2]);
    }

    fn parse_file(&mut self, filename: &str) {
        let blocks = parse_json(&self.get_filepath(filename));
        let block_length = json_value_to_usize(blocks["length"].clone());

        for idx in 0..block_length {
            let tokens = blocks[format!("token_{}", idx)].as_array().unwrap();
            let first_token = trim_quote(json_value_to_string(tokens[0].clone()));

            match first_token.as_ref() {
                "AttributeBegin" => {
                    self.pushed_graphics_state.push(self.graphics_state.clone());
                }

                "AttributeEnd" => {
                    match self.pushed_graphics_state.pop() {
                        None => {
                            panic!("unmatched `AttributeEnd` encountered.");
                        }
                        Some(_graphics_state) => {
                            self.graphics_state = _graphics_state;
                        }
                    };
                }

                "Camera" => {
                    self.parse_camera(tokens);
                }

                "CoordSysTransform" => {
                    self.parse_coord_sys_transform(tokens);
                }

                "Film" => {
                    self.parse_film(tokens);
                }

                "Include" => {
                    assert_eq!(tokens.len(), 2);
                    let included_filename = json_value_to_string(tokens[1].clone());
                    self.parse_file(&included_filename);
                }

                "LookAt" => {
                    self.parse_look_at(tokens);
                }

                "Rotate" => {
                    self.parse_rotate(tokens);
                }

                "ReverseOrientation" => {
                    self.graphics_state.reverse_orientation =
                        !self.graphics_state.reverse_orientation;
                }

                "PixelFilter" => {
                    //TODO: PixelFilter not implemented
                }

                "Scale" => {
                    self.parse_scale(tokens);
                }

                "Shape" => {
                    self.parse_shape(tokens);
                }

                "Transform" => {
                    self.parse_transform(tokens);
                }

                "Translate" => {
                    self.parse_translate(tokens);
                }

                "WorldBegin" => {
                    self.graphics_state.current_transform = Transform::identity();
                    self.named_coordinate_systems
                        .insert("world".to_string(), self.graphics_state.current_transform);
                }

                "AreaLightSource" => {
                    // TODO: parse AreaLightSource
                }

                "Filter" => {
                    //TODO: parse Filter
                }

                "Integrator" => {
                    //TODO: parse Integrator
                }

                "LightSource" => {
                    // TODO: parse LightSource
                }

                "Material" => {
                    // TODO: progress 2023/11/26 should work on Material now
                    panic!("wentao is working on building Material");
                }

                "MakeNamedMaterial" => {
                    // TODO: parse MakeNamedMaterial
                }

                "NamedMaterial" => {
                    // TODO: parse NamedMaterial
                }

                "Sampler" => {
                    //TODO: parse Sampler
                }

                "Texture" => {
                    self.parse_texture(tokens);
                }

                _ => {
                    panic!("unknown token: `{}`", first_token);
                }
            }
        }
    }

    pub fn parse_scene(&mut self, file_path: &str) -> Renderer {
        self.root = Some(get_dirname(file_path));
        self.parse_file(&get_basename(file_path));

        let filter = Arc::new(BoxFilter::new(0.5));

        let film = if self.film_entity.initialized {
            build_film(
                &self.film_entity,
                filter.clone(),
                self.global_variable.as_ref(),
            )
        } else {
            panic!("default Film not implemented");
        };

        let camera = if self.camera_entity.initialized {
            build_camera(&self.camera_entity, film.lock().unwrap().get_resolution())
        } else {
            panic!("default Camera not implemented");
        };

        let sampler = Arc::new(IndependentSampler::default());
        let bvh_aggregate = Arc::new(BVHAggregate::new(self.primitives.clone()));

        let illuminant = self.global_variable.rgb_color_space.illuminant;

        let integrator = Arc::new(AmbientOcclusion::new(illuminant, bvh_aggregate.clone()));

        return Renderer::new(integrator, sampler, camera, film);
    }
}
