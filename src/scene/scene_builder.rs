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
    file_path: String,
    graphics_state: GraphicsState,
    pushed_graphics_state: Vec<GraphicsState>,
    named_coordinate_systems: HashMap<String, Transform>,
    renderFromWorld: Transform,
    shapes: Vec<Arc<dyn Shape>>,
    camera: Option<Arc<Mutex<dyn Camera>>>,
    film: Option<Arc<Mutex<SimpleRGBFilm>>>,
}

impl SceneBuilder {
    pub fn new(_file_path: &str) -> Self {
        return SceneBuilder {
            file_path: _file_path.parse().unwrap(),
            graphics_state: GraphicsState::new(),
            pushed_graphics_state: Vec::new(),
            named_coordinate_systems: HashMap::new(),
            renderFromWorld: Transform::identity(),
            shapes: vec![],
            camera: None,
            film: None,
        };
    }

    fn RenderFromObject(&self) -> Transform {
        return self.renderFromWorld * self.graphics_state.current_transform;
    }

    fn parse_look_at(&mut self, _value: &Value) {
        println!("parsing LookAt");
        let array = _value.as_array().unwrap();
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

    fn parse_film(&mut self, _value: &Value, _filter: Arc<BoxFilter>) -> Arc<Mutex<SimpleRGBFilm>> {
        let array = _value.as_array().unwrap();
        assert_eq!(json_value_to_string(array[0].clone()), "Film");

        let name = json_value_to_string(array[1].clone());

        let parameter_dict = ParameterDict::build_from_vec(&array[2..]);
        //parameter_dict.display();

        let xresolution = parameter_dict.get_one_integer_or_panic("xresolution");
        let yresolution = parameter_dict.get_one_integer_or_panic("yresolution");

        let resolution = Point2i::new(xresolution, yresolution);
        let filename = parameter_dict.get_string_or_panic("filename");

        return Arc::new(Mutex::new(match name.as_str() {
            "rgb" => SimpleRGBFilm::new(resolution, &filename, _filter),
            &_ => {
                panic!("unknown Film name: `{}`", name);
            }
        }));
    }

    fn parse_camera(
        &mut self,
        _value: &Value,
        film: Arc<Mutex<SimpleRGBFilm>>,
    ) -> Arc<Mutex<dyn Camera>> {
        let array = _value.as_array().unwrap();
        assert_eq!(json_value_to_string(array[0].clone()), "Camera");

        let name = json_value_to_string(array[1].clone());

        let parameter_dict = ParameterDict::build_from_vec(&array[2..]);

        let camera_from_world = self.graphics_state.current_transform;
        let world_from_camera = camera_from_world.inverse();

        self.named_coordinate_systems
            .insert(String::from("camera"), camera_from_world.inverse());

        let camera_transform =
            CameraTransform::new(world_from_camera, RenderingCoordinateSystem::CameraWorld);

        self.renderFromWorld = camera_transform.RenderFromWorld();

        return Arc::new(Mutex::new(match name.as_str() {
            "perspective" => PerspectiveCamera::new(camera_transform, parameter_dict, film),
            _ => {
                panic!("unknown camera type: `{}`", name);
            }
        }));
    }

    fn parse_translate(&mut self, _value: &Value) {
        let array = _value.as_array().unwrap();
        assert_eq!(json_value_to_string(array[0].clone()), "Translate");
        assert_eq!(array.len(), 4);

        let floats: Vec<Float> = (&array.clone()[1..])
            .into_iter()
            .map(|v| json_value_to_string(v.clone()).parse::<Float>().unwrap())
            .collect();

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::translate(floats[0], floats[1], floats[2]);

        println!("`Translate` parsed");
    }

    fn parse_shape(&mut self, _value: &Value) {
        let array = _value.as_array().unwrap();
        assert_eq!(json_value_to_string(array[0].clone()), "Shape");

        let parameter_dict = ParameterDict::build_from_vec(&array[2..]);

        let renderFromObject = self.RenderFromObject();
        let objectFromRenderer = renderFromObject.inverse();

        let name = json_value_to_string(array[1].clone());
        match name.as_str() {
            "trianglemesh" => {
                let indices = parameter_dict.get_integer_array("indices");
                let mut points = parameter_dict.get_point3_array("P");

                if !renderFromObject.is_identity() {
                    for p in &mut points {
                        *p = renderFromObject.on_point3f(*p);
                    }
                }

                let triangles = build_triangles(points, indices);
                let length = triangles.len();

                for _triangle in triangles {
                    self.shapes.push(_triangle.clone());
                }

                println!(
                    "{} triangles appended, {} in total",
                    length,
                    self.shapes.len()
                );
            }
            "disk" => {
                println!("disk not implemented");
                // TODO: disk not implemented
            }
            _ => {
                panic!("unknown Shape name: `{}`", name);
            }
        };

        //parameter_dict.display();
    }

    fn parse_world_begin(&mut self, _value: &Value) {
        self.graphics_state.current_transform = Transform::identity();
        self.named_coordinate_systems
            .insert(String::from("world"), self.graphics_state.current_transform);
    }

    pub fn build_scene(&mut self) -> SceneConfig {
        let _tokens = parse_json(self.file_path.as_ref());
        let _token_length = json_value_to_usize(_tokens["length"].clone());

        let mut integrator_idx = usize::MAX;
        let mut sampler_idx = usize::MAX;
        let mut filter_idx = usize::MAX;
        let mut film_idx = usize::MAX;
        let mut camera_idx = usize::MAX;

        for idx in 0.._token_length {
            let key = format!("token_{}", idx);
            let first_token = serde_json::to_string(&_tokens[key.clone()][0]).unwrap();
            let token_without_quote = trim_quote(first_token);

            match token_without_quote.as_ref() {
                "LookAt" => {
                    self.parse_look_at(&_tokens[format!("token_{}", idx)]);
                }
                "Camera" => {
                    camera_idx = idx;
                }
                "Film" => {
                    film_idx = idx;
                }
                "Filter" => {
                    filter_idx = idx;
                    //TODO: parse Filter
                }
                "Integrator" => {
                    integrator_idx = idx;
                    //TODO: parse Integrator
                }
                "Sampler" => {
                    sampler_idx = idx;
                    //TODO: parse Sampler
                }
                "WorldBegin" => {
                    println!("before-world options parsing finished\n");

                    let filter = Arc::new(if filter_idx == usize::MAX {
                        BoxFilter::new(0.5)
                    } else {
                        panic!("Filter parsing not implemented");
                    });

                    let film =
                        self.parse_film(&_tokens[format!("token_{}", film_idx)], filter.clone());

                    self.camera = Some(
                        self.parse_camera(&_tokens[format!("token_{}", camera_idx)], film.clone()),
                    );

                    self.parse_world_begin(&_tokens[format!("token_{}", idx)]);
                }

                "AttributeBegin" => {
                    self.pushed_graphics_state.push(self.graphics_state.clone());
                }

                "AttributeEnd" => {
                    match self.pushed_graphics_state.pop() {
                        None => {
                            panic!("unmatched `AttributeEnd` encountered.");
                        }
                        Some(top_graphics_state) => {
                            self.graphics_state = top_graphics_state;
                        }
                    };
                }

                "Translate" => {
                    self.parse_translate(&_tokens[key]);
                }

                "AreaLightSource" => {
                    println!("ignore `AreaLightSource`");
                }

                "Material" => {
                    println!("ignore `Material`");
                }

                "ReverseOrientation" => {
                    self.graphics_state.reverse_orientation =
                        !self.graphics_state.reverse_orientation;
                }

                "Shape" => {
                    self.parse_shape(&_tokens[key]);
                }

                "Texture" => {
                    println!("ignore `Texture`");
                }

                _ => {
                    panic!("unknown token: `{}`", token_without_quote);
                }
            }
        }

        let camera = match &(self.camera) {
            None => {
                panic!("Camera not initialized");
            }
            Some(_camera) => _camera.clone(),
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
