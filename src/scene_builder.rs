use crate::pbrt::*;
use crate::transform::Transform;

fn json_value_to_usize(value: Value) -> usize {
    serde_json::from_value(value).unwrap()
}

fn json_value_to_string(value: Value) -> String {
    serde_json::from_value(value).unwrap()
}

fn trim_quote(token: String) -> String {
    fn head_tail(_token: &String, c: char) -> bool {
        let chars = _token.chars();
        return chars.clone().nth(0).unwrap() == c
            && chars.clone().nth(_token.len() - 1).unwrap() == c;
    }

    if head_tail(&token, '\"') || head_tail(&token, '\'') {
        return token.chars().skip(1).take(token.len() - 2).collect();
    }

    return token;
}

fn build_look_at_transform(pos: Point3f, look: Point3f, up: Vector3f) -> Transform {
    let mut worldFromCamera = SquareMatrix::<4>::default();
    worldFromCamera[0][3] = pos.x;
    worldFromCamera[1][3] = pos.y;
    worldFromCamera[2][3] = pos.z;
    worldFromCamera[3][3] = 1.0;

    let dir = (look - pos).normalize();
    if up.normalize().cross(&dir).length() == 0.0 {
        panic!("LookAt: `up` vector and viewing direction are pointing in the same direction");
    }

    let right = up.normalize().cross(&dir).normalize();
    let new_up = dir.cross(&right);

    worldFromCamera[0][0] = right.x;
    worldFromCamera[1][0] = right.y;
    worldFromCamera[2][0] = right.z;
    worldFromCamera[3][0] = 0.0;
    worldFromCamera[0][1] = new_up.x;
    worldFromCamera[1][1] = new_up.y;
    worldFromCamera[2][1] = new_up.z;
    worldFromCamera[3][1] = 0.0;
    worldFromCamera[0][2] = dir.x;
    worldFromCamera[1][2] = dir.y;
    worldFromCamera[2][2] = dir.z;
    worldFromCamera[3][2] = 0.0;

    let cameraFromWorld = worldFromCamera.inverse();
    return Transform::new_with_inv(cameraFromWorld, worldFromCamera);
}

fn parse_look_at(_value: &Value) {
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

    let transform = build_look_at_transform(position, look, up);
    println!("transform LookAt built");
}

fn parse_json(path: &str) -> Value {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("JSON was not well-formatted");
}

pub struct SceneBuilder {
    file_path: String,
}

impl SceneBuilder {
    pub fn new(_file_path: &str) -> Self {
        return SceneBuilder {
            file_path: _file_path.parse().unwrap(),
        };
    }
}

impl SceneBuilder {
    pub fn build_scene(&mut self) {
        let _tokens = parse_json(self.file_path.as_ref());
        let _token_length = json_value_to_usize(_tokens["length"].clone());

        let mut look_at_idx = usize::MAX;
        let mut camera_idx = usize::MAX;
        let mut film_idx = usize::MAX;
        let mut integrator_idx = usize::MAX;
        let mut sampler_idx = usize::MAX;

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
                    camera_idx = idx;
                    println!("matched `Camera`");
                }
                "Film" => {
                    film_idx = idx;
                    println!("matched `Film`");
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
                    println!("before-world configuration parsing finished\n");
                    break;
                }
                _ => {
                    panic!("unknown token: {}", token_without_quote)
                }
            }
        }

        // build LookAt
        // build Film
        // build camera
        // build integrator
        // build sampler

        parse_look_at(&_tokens[format!("token_{}", look_at_idx)]);

        /*
        println!("LookAt:     {}", &_tokens[format!("token_{}", look_at_idx)]);
        println!("Film:       {}", &_tokens[format!("token_{}", film_idx)]);
        println!("Camera:     {}", &_tokens[format!("token_{}", camera_idx)]);
        println!("Integrator: {}", &_tokens[format!("token_{}", integrator_idx)]);
        println!("Sampler:    {}", &_tokens[format!("token_{}", sampler_idx)]);
        */
    }
}
