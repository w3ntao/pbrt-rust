use crate::pbrt::*;

fn build_look_at_transform(pos: Point3f, look: Point3f, up: Vector3f) -> Transform {
    let mut world_from_camera = SquareMatrix::<4>::default();
    world_from_camera[0][3] = pos.x;
    world_from_camera[1][3] = pos.y;
    world_from_camera[2][3] = pos.z;
    world_from_camera[3][3] = 1.0;

    let dir = (look - pos).normalize();
    if up.normalize().cross(&dir).length() == 0.0 {
        panic!("LookAt: `up` vector and viewing direction are pointing in the same direction");
    }

    let right = up.normalize().cross(&dir).normalize();
    let new_up = dir.cross(&right);

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
}

impl SceneBuilder {
    pub fn new(_file_path: &str) -> Self {
        return SceneBuilder {
            file_path: _file_path.parse().unwrap(),
            graphics_state: GraphicsState::new(),
            pushed_graphics_state: Vec::new(),
            named_coordinate_systems: HashMap::new(),
        };
    }
}

impl SceneBuilder {
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

    fn parse_film(&mut self, _value: &Value, _filter: &BoxFilter) -> SimpleRGBFilm {
        let array = _value.as_array().unwrap();
        assert_eq!(json_value_to_string(array[0].clone()), "Film");

        let name = json_value_to_string(array[1].clone());
        println!("parsing Film: {}", name);

        let parameter_dict = ParameterDict::build_from_vec(&array[2..]);
        //parameter_dict.display();

        let xresolution = parameter_dict.get_one_integer_or_panic("xresolution");
        let yresolution = parameter_dict.get_one_integer_or_panic("yresolution");

        let resolution = Point2i::new(xresolution, yresolution);
        let filename = parameter_dict.get_string_or_panic("filename");

        match name.as_str() {
            "rgb" => {
                return SimpleRGBFilm::new(resolution, &filename, _filter.clone());
            }

            &_ => {
                panic!("unknown Film name: `{}`", name);
            }
        };
    }

    fn parse_camera(&mut self, _value: &Value, film: SimpleRGBFilm) -> PerspectiveCamera {
        let array = _value.as_array().unwrap();
        assert_eq!(json_value_to_string(array[0].clone()), "Camera");

        let name = json_value_to_string(array[1].clone());
        println!("parsing Camera: {}", name);

        let parameter_dict = ParameterDict::build_from_vec(&array[2..]);
        //parameter_dict.display();

        let camera_from_world = self.graphics_state.current_transform;
        let world_from_camera = camera_from_world.inverse();

        self.named_coordinate_systems
            .insert(String::from("camera"), camera_from_world.inverse());

        let camera_transform =
            CameraTransform::new(world_from_camera, RenderingCoordinateSystem::CameraWorld);

        return match name.as_str() {
            "perspective" => {
                println!("PerspectiveCamera built");
                PerspectiveCamera::new(camera_transform, parameter_dict)
            }
            _ => {
                panic!("unknown camera type: `{}`", name);
            }
        };
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

        for v in array {
            println!("{}", v);
        }
    }

    fn parse_world_begin(&mut self, _value: &Value) {
        self.graphics_state.current_transform = Transform::identity();
        self.named_coordinate_systems
            .insert(String::from("world"), self.graphics_state.current_transform);
    }

    pub fn build_scene(&mut self) {
        let _tokens = parse_json(self.file_path.as_ref());
        let _token_length = json_value_to_usize(_tokens["length"].clone());

        let mut look_at_idx = usize::MAX;
        let mut integrator_idx = usize::MAX;
        let mut sampler_idx = usize::MAX;
        let mut filter_idx = usize::MAX;
        let mut film_idx = usize::MAX;
        let mut camera_idx = usize::MAX;
        let mut world_begin_idx = usize::MAX;

        let mut optional_filter: Option<BoxFilter> = None;

        for idx in 0.._token_length {
            let key = format!("token_{}", idx);

            let first_token = serde_json::to_string(&_tokens[key][0]).unwrap();
            let token_without_quote = trim_quote(first_token);
            match token_without_quote.as_ref() {
                "LookAt" => {
                    look_at_idx = idx;
                    println!("matched `LookAt`");
                }
                "Camera" => {
                    println!("matched `Camera`");
                    camera_idx = idx;
                }
                "Film" => {
                    println!("matched `Film`");
                    film_idx = idx;
                }
                "Filter" => {
                    println!("matched `Filter`");
                    panic!("implement me");
                }
                "Integrator" => {
                    integrator_idx = idx;
                    println!("matched `Integrator`");
                }
                "Sampler" => {
                    sampler_idx = idx;
                    println!("matched `Sampler`");
                }
                "WorldBegin" => {
                    world_begin_idx = idx;

                    println!("before-world options parsing finished\n");
                    break;
                }
                _ => {
                    panic!("unknown token: {}", token_without_quote)
                }
            }
        }

        self.parse_look_at(&_tokens[format!("token_{}", look_at_idx)]);

        let filter = match optional_filter {
            None => BoxFilter::new(0.5),
            Some(_filter) => _filter,
        };

        let film = self.parse_film(&_tokens[format!("token_{}", film_idx)], &filter);

        let camera = self.parse_camera(&_tokens[format!("token_{}", camera_idx)], film.clone());

        self.parse_world_begin(&_tokens[format!("token_{}", world_begin_idx)]);

        println!("\n2nd part:");
        for idx in (world_begin_idx + 1).._token_length {
            let key = format!("token_{}", idx);

            let first_token = serde_json::to_string(&_tokens[key.clone()][0]).unwrap();
            let token_without_quote = trim_quote(first_token);
            //println!("{}", token_without_quote);

            match token_without_quote.as_ref() {
                "AttributeBegin" => {
                    self.pushed_graphics_state.push(self.graphics_state.clone());
                }

                "AttributeEnd" => {
                    match self.pushed_graphics_state.pop() {
                        None => {
                            panic!("Unmatched AttributeEnd encountered.");
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
    }
}
