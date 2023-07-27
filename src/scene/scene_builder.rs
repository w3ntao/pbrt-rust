use crate::pbrt::*;

fn build_look_at_transform(pos: Point3f, look: Point3f, up: Vector3f) -> Transform {
    let mut world_from_camera = SquareMatrix::<4>::default();
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
    let mut file = File::open(path).unwrap();
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

fn build_film(film_entity: &SceneEntity, _filter: Arc<BoxFilter>) -> Arc<Mutex<SimpleRGBFilm>> {
    let xresolution = film_entity.parameters.get_one_integer("xresolution", None);
    let yresolution = film_entity.parameters.get_one_integer("yresolution", None);

    let resolution = Point2i::new(xresolution, yresolution);
    let filename = film_entity.parameters.get_string_or_panic("filename");

    return Arc::new(Mutex::new(match film_entity.name.as_str() {
        "rgb" => SimpleRGBFilm::new(resolution, &filename, _filter),
        &_ => {
            panic!("unknown Film name: `{}`", film_entity.name);
        }
    }));
}

fn build_camera(
    camera_entity: &SceneEntity,
    film: Arc<Mutex<SimpleRGBFilm>>,
) -> Arc<Mutex<dyn Camera>> {
    return Arc::new(Mutex::new(match camera_entity.name.as_str() {
        "perspective" => PerspectiveCamera::new(
            camera_entity.camera_transform,
            camera_entity.parameters.clone(),
            film,
        ),
        _ => {
            panic!("unknown camera type: `{}`", camera_entity.name);
        }
    }));
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
    renderFromWorld: Transform,
    shapes: Vec<Arc<dyn Shape>>,

    film_entity: SceneEntity,
    camera_entity: SceneEntity,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        return SceneBuilder {
            graphics_state: GraphicsState::new(),
            pushed_graphics_state: Vec::new(),
            named_coordinate_systems: HashMap::new(),
            renderFromWorld: Transform::identity(),
            shapes: vec![],

            film_entity: SceneEntity::default(),
            camera_entity: SceneEntity::default(),
        };
    }
}

impl SceneBuilder {
    fn RenderFromObject(&self) -> Transform {
        return self.renderFromWorld * self.graphics_state.current_transform;
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
        println!("transform LookAt built\n");

        self.graphics_state.current_transform =
            self.graphics_state.current_transform * transform_look_at;
    }

    fn parse_film(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "Film");

        self.film_entity.initialized = true;
        self.film_entity.name = json_value_to_string(tokens[1].clone());
        self.film_entity.parameters = ParameterDict::build_from_vec(&tokens[2..]);
    }

    fn parse_camera(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "Camera");

        let name = json_value_to_string(tokens[1].clone());
        let parameter_dict = ParameterDict::build_from_vec(&tokens[2..]);

        let camera_from_world = self.graphics_state.current_transform;
        let world_from_camera = camera_from_world.inverse();

        self.named_coordinate_systems
            .insert(String::from("camera"), camera_from_world.inverse());

        let camera_transform =
            CameraTransform::new(world_from_camera, RenderingCoordinateSystem::CameraWorld);

        self.renderFromWorld = camera_transform.RenderFromWorld();

        self.camera_entity.initialized = true;
        self.camera_entity.name = name;
        self.camera_entity.camera_transform = camera_transform;
        self.camera_entity.parameters = parameter_dict;
    }

    fn parse_translate(&mut self, tokens: &Vec<Value>) {
        assert_eq!(tokens.len(), 4);
        assert_eq!(json_value_to_string(tokens[0].clone()), "Translate");

        let floats: Vec<Float> = (&tokens.clone()[1..])
            .into_iter()
            .map(|v| json_value_to_string(v.clone()).parse::<Float>().unwrap())
            .collect();

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::translate(floats[0], floats[1], floats[2]);

        println!("`Translate` parsed");
    }

    fn parse_shape(&mut self, tokens: &Vec<Value>) {
        assert_eq!(json_value_to_string(tokens[0].clone()), "Shape");

        let parameters = ParameterDict::build_from_vec(&tokens[2..]);

        let renderFromObject = self.RenderFromObject();
        let objectFromRenderer = renderFromObject.inverse();

        let name = json_value_to_string(tokens[1].clone());
        match name.as_str() {
            "disk" => {
                println!("disk not implemented");
                // TODO: disk not implemented
            }

            "loopsubdiv" => {
                println!("loopsubdiv not implemented");

                let levels = parameters.get_one_integer("levels", Some(3)) as usize;
                let indices_i32 = parameters.get_integer_array("indices");
                let indices = indices_i32.into_iter().map(|x| x as usize).collect();

                let points = parameters.get_point3_array("P");

                let triangles = loop_subdivide(
                    renderFromObject,
                    self.graphics_state.reverse_orientation,
                    levels,
                    indices,
                    points,
                );
            }

            "trianglemesh" => {
                let indices = parameters.get_integer_array("indices");
                let mut points = parameters.get_point3_array("P");

                let mesh = TriangleMesh::new(
                    renderFromObject,
                    points,
                    indices.into_iter().map(|x| x as usize).collect(),
                );

                let triangles = mesh.create_triangles();
                for _triangle in &triangles {
                    self.shapes.push(_triangle.clone());
                }

                println!(
                    "{} triangles appended, {} in total",
                    triangles.len(),
                    self.shapes.len()
                );
            }
            _ => {
                panic!("unknown Shape name: `{}`", name);
            }
        };
    }

    fn parse_file(&mut self, file_path: &str) {
        let blocks = parse_json(file_path);
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

                "LookAt" => {
                    self.parse_look_at(tokens);
                }

                "Film" => {
                    self.parse_film(tokens);
                }
                "Filter" => {
                    println!("parsing Filter not implemented\n");
                    //TODO: parse Filter
                }
                "Integrator" => {
                    println!("parsing Integrator not implemented\n");
                    //TODO: parse Integrator
                }
                "Sampler" => {
                    println!("parsing Sampler not implemented\n");
                    //TODO: parse Sampler
                }
                "WorldBegin" => {
                    println!("before-world options parsing finished\n");
                    self.graphics_state.current_transform = Transform::identity();
                    self.named_coordinate_systems
                        .insert(String::from("world"), self.graphics_state.current_transform);
                }

                "Translate" => {
                    self.parse_translate(tokens);
                }

                "AreaLightSource" => {
                    println!("ignore `AreaLightSource`");
                }

                "Include" => {
                    let included_path = json_value_to_string(tokens[1].clone());
                    let absolute_path =
                        format!("{}/{}", get_folder_potion(file_path), included_path);
                    println!("{}", absolute_path);
                    self.parse_file(absolute_path.as_str());
                }

                "LightSource" => {
                    println!("ignore `LightSource`");
                }

                "Material" => {
                    println!("ignore `Material`");
                }

                "ReverseOrientation" => {
                    self.graphics_state.reverse_orientation =
                        !self.graphics_state.reverse_orientation;
                }

                "Shape" => {
                    self.parse_shape(tokens);
                }

                "Texture" => {
                    println!("ignore `Texture`");
                }
                _ => {
                    panic!("unknown token: `{}`", first_token);
                }
            }
        }
    }

    pub fn parse_scene(&mut self, file_path: &str) -> SceneConfig {
        self.parse_file(file_path);

        let filter = Arc::new(BoxFilter::new(0.5));

        let film = if self.film_entity.initialized {
            build_film(&self.film_entity, filter.clone())
        } else {
            panic!("default Film not implemented");
        };

        let camera = if self.camera_entity.initialized {
            build_camera(&self.camera_entity, film.clone())
        } else {
            panic!("default Camera not implemented");
        };

        let sampler = Arc::new(IndependentSampler::default());

        let integrator = Arc::new(SurfaceNormalVisualizer::new());

        return SceneConfig::new(
            integrator.clone(),
            camera.clone(),
            sampler.clone(),
            self.shapes.clone(),
        );
    }
}
