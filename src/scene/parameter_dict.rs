use crate::pbrt::*;

pub struct ParameterDict {
    integers: HashMap<String, Vec<i32>>,
    floats: HashMap<String, Vec<Float>>,
    strings: HashMap<String, String>,
    point2s: HashMap<String, Vec<Point2f>>,
    point3s: HashMap<String, Vec<Point3f>>,
    normal3s: HashMap<String, Vec<Normal3f>>,
    rgbs: HashMap<String, RGB>,
    textures: HashMap<String, Arc<dyn SpectrumTexture>>,
    bools: HashMap<String, Vec<bool>>,
}

impl Default for ParameterDict {
    fn default() -> Self {
        return ParameterDict {
            integers: HashMap::new(),
            floats: HashMap::new(),
            strings: HashMap::new(),
            point2s: HashMap::new(),
            point3s: HashMap::new(),
            normal3s: HashMap::new(),
            rgbs: HashMap::new(),
            textures: HashMap::new(),
            bools: HashMap::new(),
        };
    }
}

impl Clone for ParameterDict {
    fn clone(&self) -> Self {
        return ParameterDict {
            integers: self.integers.clone(),
            floats: self.floats.clone(),
            strings: self.strings.clone(),
            point2s: self.point2s.clone(),
            point3s: self.point3s.clone(),
            normal3s: self.normal3s.clone(),
            rgbs: self.rgbs.clone(),
            textures: self.textures.clone(),
            bools: self.bools.clone(),
        };
    }
}

pub fn split_variable_type_name(token: String) -> (String, String) {
    let parts = token.split_whitespace().collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);

    return (parts[0].to_string(), parts[1].to_string());
}

pub fn fetch_variable_value(value: &Value) -> Vec<String> {
    return match value.as_array() {
        None => {
            // if it's a string
            vec![value.as_str().unwrap().to_string()]
        }
        Some(value_vector) => {
            // if it's a vec of string
            value_vector
                .into_par_iter()
                .map(|v| json_value_to_string(v.clone()))
                .collect()
        }
    };
}

fn _print<T: Display>(hashmap: &HashMap<String, Vec<T>>) {
    for (k, values) in hashmap {
        print!("{} -> ", k);
        print!("[ ");
        for v in values {
            print!("{} ", v);
        }
        print!("]\n");
    }
    println!();
}

pub fn convert_string<T: FromStr>(string_vec: &Vec<String>) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    return string_vec
        .into_iter()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>();
}

fn get_one_val<T: Copy>(key: &str, default: Option<T>, dict: &HashMap<String, Vec<T>>) -> T {
    match (dict.get(key), default) {
        (Some(val), _) => {
            if val.len() != 1 {
                panic!("array length is larger than 1");
            }
            return val[0];
        }
        (_, Some(val)) => {
            return val;
        }
        _ => {
            panic!("found no key with name `{}`", key);
        }
    }
}

fn get_array<T: Copy>(key: &str, dict: &HashMap<String, Vec<T>>) -> Vec<T> {
    return match dict.get(key) {
        None => {
            panic!("found no key with name `{}`", key);
        }
        Some(val) => val.clone(),
    };
}

impl ParameterDict {
    pub fn build_parameter_dict(
        array: &[Value],
        named_texture: &HashMap<String, Arc<dyn SpectrumTexture>>,
        dir_path: Option<String>,
    ) -> ParameterDict {
        let mut integers = HashMap::<String, Vec<i32>>::new();
        let mut floats = HashMap::<String, Vec<Float>>::new();
        let mut strings = HashMap::<String, String>::new();
        let mut point2s = HashMap::<String, Vec<Point2f>>::new();
        let mut point3s = HashMap::<String, Vec<Point3f>>::new();
        let mut normal3s = HashMap::<String, Vec<Normal3f>>::new();
        let mut rgbs = HashMap::<String, RGB>::new();
        let mut textures = HashMap::<String, Arc<dyn SpectrumTexture>>::new();
        let mut bools = HashMap::<String, Vec<bool>>::new();

        for idx in (0..array.len()).step_by(2) {
            let token = trim_quote(json_value_to_string(array[idx].clone()));
            let (variable_type, variable_name) = split_variable_type_name(token);

            let variable_values = fetch_variable_value(&array[idx + 1]);

            match variable_type.as_str() {
                "string" => {
                    assert_eq!(variable_values.len(), 1);

                    match (variable_name.as_str(), &dir_path) {
                        ("filename", Some(dir)) => {
                            strings.insert(
                                variable_name,
                                format!("{}/{}", dir, variable_values[0].clone()),
                            );
                        }
                        (_, _) => {
                            strings.insert(variable_name, variable_values[0].clone());
                        }
                    };
                }
                "integer" => {
                    integers.insert(variable_name, convert_string::<i32>(&variable_values));
                }
                "float" => {
                    floats.insert(variable_name, convert_string::<Float>(&variable_values));
                }
                "point2" => {
                    let float_numbers = convert_string::<Float>(&variable_values);

                    let mut point_set = vec![];
                    for idx in (0..float_numbers.len()).step_by(2) {
                        point_set.push(Point2f::new(float_numbers[idx], float_numbers[idx + 1]));
                    }
                    point2s.insert(variable_name, point_set);
                }
                "point3" => {
                    let float_numbers = convert_string::<Float>(&variable_values);

                    let mut point_set = vec![];
                    for idx in (0..float_numbers.len()).step_by(3) {
                        point_set.push(Point3f::new(
                            float_numbers[idx],
                            float_numbers[idx + 1],
                            float_numbers[idx + 2],
                        ));
                    }
                    point3s.insert(variable_name, point_set);
                }

                "normal" => {
                    let float_numbers = convert_string::<Float>(&variable_values);

                    let mut normal_set = vec![];
                    for idx in (0..float_numbers.len()).step_by(3) {
                        normal_set.push(Normal3f::new(
                            float_numbers[idx],
                            float_numbers[idx + 1],
                            float_numbers[idx + 2],
                        ));
                    }
                    normal3s.insert(variable_name, normal_set);
                }

                "rgb" => {
                    let float_numbers = convert_string::<Float>(&variable_values);
                    assert_eq!(float_numbers.len(), 3);

                    rgbs.insert(
                        variable_name,
                        RGB::new(float_numbers[0], float_numbers[1], float_numbers[2]),
                    );
                }

                "texture" => {
                    assert_eq!(variable_values.len(), 1);
                    let texture_id = variable_values[0].clone();

                    let texture = match named_texture.get(&texture_id) {
                        None => {
                            panic!("texture not found: `{}`", texture_id);
                        }
                        Some(_texture) => _texture.clone(),
                    };

                    textures.insert(variable_name, texture);
                }

                _ => {
                    panic!("unknown variable type: `{}`", variable_type);
                }
            }
        }

        return Self {
            integers,
            floats,
            strings,
            point2s,
            point3s,
            normal3s,
            rgbs,
            textures,
            bools,
        };
    }

    pub fn has_rgb(&self, name: &str) -> bool {
        return self.rgbs.get(name).is_some();
    }

    pub fn has_texture(&self, name: &str) -> bool {
        return self.textures.get(name).is_some();
    }

    pub fn insert_integer(&mut self, name: String, value: Vec<i32>) {
        if self.integers.contains_key(&name) {
            panic!("duplicate key: `{}`", name);
        }

        self.integers.insert(name, value);
    }

    pub fn insert_float(&mut self, name: String, value: Vec<Float>) {
        if self.floats.contains_key(&name) {
            panic!("duplicate key: `{}`", name);
        }

        self.floats.insert(name, value);
    }

    pub fn insert_string(&mut self, name: String, value: String) {
        if self.strings.contains_key(&name) {
            panic!("duplicate key: `{}`", name);
        }

        self.strings.insert(name, value);
    }

    pub fn get_rgb(&self, key: &str, default: Option<RGB>) -> RGB {
        return match (self.rgbs.get(key), default) {
            (None, Some(val)) => val,
            (Some(val), _) => val.clone(),
            _ => {
                panic!("get_rgb(): found no key with name `{}`", key);
            }
        };
    }

    pub fn get_string(&self, key: &str, default: Option<String>) -> String {
        return match (self.strings.get(key), default) {
            (None, Some(val)) => val,
            (Some(val), _) => val.clone(),
            _ => {
                panic!("get_string(): found no key with name `{}`", key);
            }
        };
    }

    pub fn get_texture(&self, key: &str) -> Arc<dyn SpectrumTexture> {
        return match self.textures.get(key) {
            Some(val) => val.clone(),
            _ => {
                panic!("get_texture(): found no key with name `{}`", key);
            }
        };
    }
    pub fn get_one_float(&self, key: &str, default: Option<Float>) -> Float {
        return get_one_val(key, default, &self.floats);
    }

    pub fn get_one_integer(&self, key: &str, default: Option<i32>) -> i32 {
        return get_one_val(key, default, &self.integers);
    }

    pub fn get_one_bool(&self, key: &str, default: Option<bool>) -> bool {
        return get_one_val(key, default, &self.bools);
    }

    pub fn get_integer_array(&self, key: &str) -> Vec<i32> {
        return get_array(key, &self.integers);
    }

    pub fn get_point2_array(&self, key: &str) -> Vec<Point2f> {
        return get_array(key, &self.point2s);
    }

    pub fn get_one_point3(&self, key: &str, default: Option<Point3f>) -> Point3f {
        return match (self.point3s.get(key), default) {
            (Some(val), _) => {
                if val.len() == 1 {
                    val[0]
                } else {
                    unreachable!();
                }
            }
            (None, Some(val)) => val,
            _ => {
                unreachable!();
            }
        };
    }

    pub fn get_point3_array(&self, key: &str) -> Vec<Point3f> {
        return get_array(key, &self.point3s);
    }

    pub fn get_normal3_array(&self, key: &str) -> Vec<Normal3f> {
        return match self.normal3s.get(key) {
            None => {
                return vec![];
            }
            Some(val) => val.clone(),
        };
    }
}

//floats: HashMap<String, Vec<Float>>,
fn display_single_value_dict<T: Display>(dict: &HashMap<String, T>) {
    for (key, value) in dict {
        println!("    {} -> {}", key, value);
    }
}

fn display_multi_value_dict<T: Display>(dict: &HashMap<String, Vec<T>>) {
    for (key, value) in dict {
        print!("    {} -> [ ", key);
        for v in value {
            print!("{}, ", v);
        }
        println!("]");
    }
}

impl Display for ParameterDict {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "strings: {}\n", self.strings.len()).unwrap();
        display_single_value_dict(&self.strings);

        write!(f, "integers: {}\n", self.integers.len()).unwrap();
        display_multi_value_dict(&self.integers);

        write!(f, "floats: {}\n", self.floats.len()).unwrap();
        display_multi_value_dict(&self.floats);

        write!(f, "point2s: {}\n", self.point2s.len()).unwrap();
        display_multi_value_dict(&self.point2s);

        write!(f, "point3s: {}\n", self.point3s.len()).unwrap();
        display_multi_value_dict(&self.point3s);

        write!(f, "normal3s: {}\n", self.normal3s.len()).unwrap();
        display_multi_value_dict(&self.normal3s);

        write!(f, "rgbs: {}\n", self.rgbs.len()).unwrap();
        display_single_value_dict(&self.rgbs);

        write!(f, "textures: {}\n", self.textures.len()).unwrap();
        for key in self.textures.keys() {
            println!("    {}", key);
        }

        write!(f, "bools: {}\n", self.bools.len()).unwrap();
        display_multi_value_dict(&self.bools);

        return Ok(());
    }
}
