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

struct SceneEntity {
    pub name: String,
    pub parameters: ParameterDict,
}

struct CameraEntity {
    pub name: String,
    pub parameters: ParameterDict,
    pub camera_transform: CameraTransform,
}

struct LightEntity {
    pub name: String,
    pub parameters: ParameterDict,
    pub render_from_object: Transform,
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

fn build_camera(camera_entity: &CameraEntity, resolution: Point2i) -> Arc<dyn Camera> {
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

fn build_lights(
    light_entities: &Vec<LightEntity>,
    global_variable: &GlobalVariable,
) -> Vec<Arc<dyn Light>> {
    /*
    if light_entities.len() == 0 {
        panic!("no light available");
    }
    */

    let mut lights: Vec<Arc<dyn Light>> = vec![];

    for light_entity in light_entities {
        let light_type = light_entity.name.as_str();
        match light_type {
            "distant" => {
                let light = DistantLight::new(
                    &light_entity.render_from_object,
                    &light_entity.parameters,
                    global_variable.rgb_color_space.as_ref(),
                );

                lights.push(Arc::new(light));
            }
            _ => {
                panic!("`{}` not implemented", light_type)
            }
        }
    }

    return lights;
}

fn build_integrator(
    aggregate: Arc<dyn Primitive>,
    camera: Arc<dyn Camera>,
    lights: Vec<Arc<dyn Light>>,
    global_variable: &GlobalVariable,
) -> Arc<dyn Integrator> {
    let integrator = AmbientOcclusion::new(
        global_variable.rgb_color_space.illuminant,
        aggregate,
        camera,
    );

    /*
    let integrator = RandomWalkIntegrator::new(
        global_variable.rgb_color_space.illuminant,
        aggregate,
        camera,
        lights,
    );
    */

    /*
    let integrator =
        SurfaceNormal::new(aggregate, camera, global_variable.rgb_color_space.as_ref());
    */

    return Arc::new(integrator);
}

fn split_tokens_into_statements(tokens: &[Token]) -> Vec<usize> {
    let mut keyword_range = vec![];
    for idx in 0..tokens.len() {
        match tokens[idx].clone() {
            Token::WorldBegin | Token::AttributeEnd | Token::AttributeBegin | Token::Keyword(_) => {
                keyword_range.push(idx);
            }
            _ => {}
        }
    }
    keyword_range.push(tokens.len());

    return keyword_range;
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

    film_entity: Option<SceneEntity>,
    camera_entity: Option<CameraEntity>,
    light_entities: Vec<LightEntity>,

    current_material: Option<Arc<dyn Material>>,

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

            film_entity: None,
            camera_entity: None,
            current_material: None,
            light_entities: vec![],

            root: None,
            global_variable,
        };
    }

    fn world_area_light_source(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("AreaLightSource".to_string()));

        let name = tokens[1].convert_to_string();
        if name != "diffuse" {
            panic!("only `diffuse` Area Light is supported");
        }

        let parameter_dict = ParameterDict::build_parameter_dict(
            &tokens[2..],
            &self.named_texture,
            self.root.clone(),
        );

        // TODO: progress 2023/12/14: having parser/lexer trouble when implementing DiffuseAreaLight

        panic!("world_area_light_source() not implemented");
    }

    fn world_coord_sys_transform(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("CoordSysTransform".to_string()));
        debug_assert!(tokens.len() == 2);

        let coord_sys_name = {
            let token = tokens[1].clone();
            match token {
                Token::String(kw) => kw,
                _ => {
                    panic!("expect Token::String, get `{:?}`", token);
                }
            }
        };

        self.graphics_state.current_transform =
            match self.named_coordinate_systems.get(&coord_sys_name) {
                None => {
                    panic!("couldn't find key {}", coord_sys_name);
                }
                Some(transform) => *transform,
            };
    }

    fn world_light_source(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("LightSource".to_string()));

        let light_source_type = tokens[1].convert_to_string();

        let light_entity = LightEntity {
            name: light_source_type,
            parameters: ParameterDict::build_parameter_dict(
                &tokens[2..],
                &self.named_texture,
                None,
            ),
            render_from_object: self.render_from_object(),
        };

        self.light_entities.push(light_entity);
    }
    fn world_material(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0] == Token::Keyword("Material".to_string()));

        let parameter_dict = ParameterDict::build_parameter_dict(
            &tokens[2..],
            &self.named_texture,
            self.root.clone(),
        );

        let material_type = tokens[1].convert_to_string();
        self.current_material = Some(create_material(&material_type, &parameter_dict));
    }

    fn world_rotate(&mut self, tokens: &[Token]) {
        debug_assert!(tokens.len() == 5);
        debug_assert!(tokens[0].clone() == Token::Keyword("Rotate".to_string()));

        let floats = tokens[1..]
            .into_iter()
            .map(|t| t.convert_to_float())
            .collect::<Vec<f64>>();

        debug_assert!(floats.len() == 4);

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::rotate(floats[0], floats[1], floats[2], floats[3]);
    }

    fn world_scale(&mut self, tokens: &[Token]) {
        debug_assert!(tokens.len() == 4);
        debug_assert!(tokens[0].clone() == Token::Keyword("Scale".to_string()));

        let floats = tokens[1..]
            .into_iter()
            .map(|t| t.convert_to_float())
            .collect::<Vec<f64>>();
        debug_assert!(floats.len() == 4);

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::scale(floats[0], floats[1], floats[2]);
    }

    fn world_shape(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("Shape".to_string()));

        let material = match &self.current_material {
            None => {
                panic!("current material is not available");
            }
            Some(_material) => _material.clone(),
        };

        let parameters = ParameterDict::build_parameter_dict(
            &tokens[2..],
            &self.named_texture,
            self.root.clone(),
        );

        let render_from_object = self.render_from_object();
        let object_from_render = render_from_object.inverse();

        let reverse_orientation = self.graphics_state.reverse_orientation;

        let name = tokens[1].convert_to_string();
        match name.as_str() {
            "loopsubdiv" => {
                let levels = parameters.get_one_integer("levels", Some(3)) as usize;
                let indices_i32 = parameters.get_integer_array("indices");
                let indices = indices_i32.into_par_iter().map(|x| x as usize).collect();

                let points = parameters.get_point3_array("P");

                let triangles = loop_subdivide(&render_from_object, levels, indices, points);

                for _triangle in &triangles {
                    let primitive = SimplePrimitive::new(_triangle.clone(), material.clone());
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

                let primitive = SimplePrimitive::new(Arc::new(sphere), material.clone());
                self.primitives.push(Arc::new(primitive));
            }

            "trianglemesh" => {
                let indices = parameters.get_integer_array("indices");
                let points = parameters.get_point3_array("P");
                let normals = parameters.get_normal3_array("N");
                let uv = if parameters.has_point2("uv") {
                    parameters.get_point2_array("uv")
                } else {
                    vec![]
                };

                let mesh = TriangleMesh::new(
                    &render_from_object,
                    points,
                    indices.into_par_iter().map(|x| x as usize).collect(),
                    normals,
                    uv,
                );

                let triangles = mesh.create_triangles();
                for _triangle in &triangles {
                    let primitive = SimplePrimitive::new(_triangle.clone(), material.clone());
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
                        tri_quad_mesh.uv,
                    );

                    let triangles = triangle_mesh.create_triangles();
                    for _triangle in &triangles {
                        let primitive = SimplePrimitive::new(_triangle.clone(), material.clone());
                        self.primitives.push(Arc::new(primitive));
                    }
                }
            }
            _ => {
                panic!("unknown Shape: `{}`", name);
            }
        };
    }

    fn world_texture(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("Texture".to_string()));

        let texture_name = tokens[1].convert_to_string();
        let color_type = tokens[2].convert_to_string();
        let texture_type = tokens[3].convert_to_string();

        match color_type.as_str() {
            "spectrum" => {
                let parameter_dict = ParameterDict::build_parameter_dict(
                    &tokens[4..],
                    &self.named_texture,
                    self.root.clone(),
                );

                //TODO: SpectrumType is missing in creating SpectrumTexture
                let texture = create_spectrum_texture(
                    &texture_type,
                    &self.render_from_object(),
                    &parameter_dict,
                    SpectrumType::Albedo,
                    self.global_variable.as_ref(),
                );
                // TODO: hardcode all SpectrumType as Albedo for the moment

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

    fn world_transform(&mut self, tokens: &[Token]) {
        debug_assert!(tokens.len() == 2);
        debug_assert!(tokens[0].clone() == Token::Keyword("Transform".to_string()));

        let floats = tokens[1..]
            .into_iter()
            .map(|t| t.convert_to_float())
            .collect::<Vec<f64>>();
        debug_assert!(floats.len() == 16);

        self.graphics_state.current_transform =
            Transform::from_matrix(SquareMatrix::<4>::from_array(&floats)).transpose();
    }

    fn world_translate(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("Translate".to_string()));
        debug_assert!(tokens.len() == 4);

        let floats = tokens[1..]
            .into_iter()
            .map(|t| t.convert_to_float())
            .collect::<Vec<f64>>();

        self.graphics_state.current_transform = self.graphics_state.current_transform
            * Transform::translate(floats[0], floats[1], floats[2]);
    }

    fn parse_statement(&mut self, tokens: &[Token]) {
        let first_token = tokens[0].clone();

        match first_token {
            Token::AttributeBegin => {
                assert_eq!(tokens.len(), 1);
                self.pushed_graphics_state.push(self.graphics_state.clone());
            }

            Token::AttributeEnd => {
                assert_eq!(tokens.len(), 1);

                match self.pushed_graphics_state.pop() {
                    None => {
                        panic!("unmatched `AttributeEnd` encountered.");
                    }
                    Some(_graphics_state) => {
                        self.graphics_state = _graphics_state;
                    }
                };
            }

            Token::WorldBegin => {
                // WorldBegin
                self.graphics_state.current_transform = Transform::identity();
                self.named_coordinate_systems
                    .insert("world".to_string(), self.graphics_state.current_transform);
            }

            Token::Keyword(keyword) => {
                match keyword.as_str() {
                    "AreaLightSource" => {
                        self.world_area_light_source(tokens);
                    }

                    "Camera" => {
                        self.option_camera(tokens);
                    }

                    "CoordSysTransform" => {
                        self.world_coord_sys_transform(tokens);
                    }

                    "Film" => {
                        self.option_film(tokens);
                    }

                    "Include" => {
                        assert_eq!(tokens.len(), 2);
                        let included_filename = tokens[1].convert_to_string();
                        self.parse_file(&included_filename);
                    }

                    "Integrator" => {
                        // TODO: parse Integrator
                        println!("parse_Integrator() not implemented");
                    }

                    "LightSource" => {
                        self.world_light_source(tokens);
                    }

                    "LookAt" => {
                        self.option_look_at(tokens);
                    }

                    "Material" => {
                        self.world_material(tokens);
                    }

                    "ReverseOrientation" => {
                        self.graphics_state.reverse_orientation =
                            !self.graphics_state.reverse_orientation;
                    }

                    "Rotate" => {
                        self.world_rotate(tokens);
                    }

                    "Scale" => {
                        self.world_scale(tokens);
                    }

                    "Sampler" => {
                        // TODO: parse Sampler
                        println!("parse_sampler() not implemented");
                    }

                    "Shape" => {
                        self.world_shape(tokens);
                    }

                    "Texture" => {
                        self.world_texture(tokens);
                    }

                    "Transform" => {
                        self.world_transform(tokens);
                    }

                    "Translate" => {
                        self.world_translate(tokens);
                    }

                    _ => {
                        panic!("unrecognized keyword: `{:?}`", keyword);
                    }
                };
            }
            _ => {
                panic!("unrecognized Token: `{:?}`", first_token);
            }
        }
    }

    fn option_camera(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("Camera".to_string()));

        let camera_type = match tokens[1].clone() {
            Token::String(str) => str,
            _ => {
                panic!("expect Token::String");
            }
        };

        let parameter_dict = ParameterDict::build_parameter_dict(
            &tokens[2..],
            &self.named_texture,
            self.root.clone(),
        );

        let camera_from_world = self.graphics_state.current_transform;
        let world_from_camera = camera_from_world.inverse();

        self.named_coordinate_systems
            .insert("camera".to_string(), camera_from_world.inverse());

        let camera_transform =
            CameraTransform::new(world_from_camera, RenderingCoordinateSystem::CameraWorld);

        self.render_from_world = camera_transform.render_from_world();

        self.camera_entity = Some(CameraEntity {
            name: camera_type.to_string(),
            parameters: parameter_dict,
            camera_transform,
        });
    }

    fn option_film(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("Film".to_string()));

        let film_type = match tokens[1].clone() {
            Token::String(str) => {
                if str != "rgb" {
                    println!("warning: only `rgb` film is supported for the moment.");
                }
                "rgb".to_string()
            }
            _ => {
                panic!("expect Token::String");
            }
        };

        self.film_entity = Some(SceneEntity {
            name: film_type,
            parameters: ParameterDict::build_parameter_dict(
                &tokens[2..],
                &self.named_texture,
                None,
            ),
        });
    }

    fn option_look_at(&mut self, tokens: &[Token]) {
        debug_assert!(tokens[0].clone() == Token::Keyword("LookAt".to_string()));

        let data: Vec<f64> = tokens[1..]
            .into_iter()
            .map(|token| match token {
                Token::Number(num) => num.parse::<f64>().unwrap(),
                _ => {
                    panic!("expect Token::Number here");
                }
            })
            .collect();

        let position = Point3f::new(data[0], data[1], data[2]);
        let look = Point3f::new(data[3], data[4], data[5]);
        let up = Vector3f::new(data[6], data[7], data[8]);

        let transform_look_at = build_look_at_transform(position, look, up);

        self.graphics_state.current_transform =
            self.graphics_state.current_transform * transform_look_at;
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

    fn parse_file(&mut self, filename: &str) {
        let tokens = parse_pbrt_into_token(&self.get_filepath(filename));

        let range_of_statement = split_tokens_into_statements(&tokens);

        for range_idx in 0..(range_of_statement.len() - 1) {
            let statement =
                &tokens[range_of_statement[range_idx]..range_of_statement[range_idx + 1]];

            self.parse_statement(statement);
        }
    }

    pub fn parse_scene(&mut self, file_path: &str, samples_per_pixel: usize) -> Renderer {
        self.root = Some(get_dirname(file_path));
        self.parse_file(&get_basename(file_path));

        let filter = Arc::new(BoxFilter::new(0.5));

        let film = match &self.film_entity {
            None => {
                panic!("default Film not implemented");
            }
            Some(film_entity) => {
                build_film(&film_entity, filter.clone(), self.global_variable.as_ref())
            }
        };

        let camera = match &self.camera_entity {
            None => {
                panic!("default Camera not implemented");
            }
            Some(camera_entity) => {
                build_camera(&camera_entity, film.lock().unwrap().get_resolution())
            }
        };

        let sampler = Arc::new(IndependentSampler::new(samples_per_pixel));
        let bvh_aggregate = Arc::new(BVHAggregate::new(self.primitives.clone()));

        let lights = build_lights(&self.light_entities, &self.global_variable);

        let integrator = build_integrator(
            bvh_aggregate,
            camera.clone(),
            lights,
            self.global_variable.as_ref(),
        );

        return Renderer::new(integrator, sampler, camera, film);
    }
}
