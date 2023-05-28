use std::fs::File;
use std::io::Read;
use serde_json::Value;

fn json_value_to_usize(value: Value) -> usize {
    serde_json::from_value(value).unwrap()
}

fn json_value_to_string(value: Value) -> String {
    serde_json::from_value(value).unwrap()
}

fn trim_quote(token: String) -> String {
    fn head_tail(_token: &String, c: char) -> bool {
        let chars = _token.chars();
        return chars.clone().nth(0).unwrap() == c && chars.clone().nth(_token.len() - 1).unwrap() == c;
    }

    if head_tail(&token, '\"') || head_tail(&token, '\'') {
        return token.chars().skip(1).take(token.len() - 2).collect();
    }

    return token;
}

fn build_look_at(_value: &Value) {
    let array = _value.as_array().unwrap();
    assert_eq!(json_value_to_string(array[0].clone()), "LookAt");

    let length = array.len();
    assert_eq!(length, 10);

    let mut matrix = [f32::NAN; 9];
    for idx in 1..length {
        let number_in_string = trim_quote(json_value_to_string(array[idx].clone()));

        matrix[idx - 1] = number_in_string.parse::<f32>().unwrap();
    }

    print!("matrix `LookAt`: ");
    for v in matrix {
        print!("{} ", v);
    }
    println!();
}

struct Integrator {}

struct Camera {}

struct Film {}

struct Sampler {}

pub struct SceneBuilder {
    tokens: Value,
    token_length: usize,
    // dependency: Camera -> Film -> Filter

    /*
    look_at: [f32; 9],
    camera: Arc<Camera>,
    film: Arc<Film>,
    integrator: Arc<Integrator>,
    sampler: Arc<Sampler>,
    world_start_idx: usize,
     */
}

fn parse_json(path: &str) -> Value {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("JSON was not well-formatted");
}

impl SceneBuilder {
    pub fn build_from_json(file_path: &str) -> Self {
        let _tokens = parse_json(file_path);
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
                _ => { panic!("unknown token: {}", token_without_quote) }
            }
        }

        // build LookAt
        // build Film
        // build camera
        // build integrator
        // build sampler

        build_look_at(&_tokens[format!("token_{}", look_at_idx)]);

        /*
        println!("LookAt:     {}", &_tokens[format!("token_{}", look_at_idx)]);
        println!("Film:       {}", &_tokens[format!("token_{}", film_idx)]);
        println!("Camera:     {}", &_tokens[format!("token_{}", camera_idx)]);
        println!("Integrator: {}", &_tokens[format!("token_{}", integrator_idx)]);
        println!("Sampler:    {}", &_tokens[format!("token_{}", sampler_idx)]);
        */

        return Self {
            tokens: _tokens,
            token_length: _token_length,
        };
    }
}
