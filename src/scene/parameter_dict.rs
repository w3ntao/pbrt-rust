use crate::pbrt::*;

pub struct ParameterDict {
    integers: HashMap<String, Vec<i32>>,
    floats: HashMap<String, Vec<Float>>,
    strings: HashMap<String, String>,
    point2s: HashMap<String, Vec<Point2f>>,
    point3s: HashMap<String, Vec<Point3f>>,
}

impl Default for ParameterDict {
    fn default() -> Self {
        return ParameterDict {
            integers: HashMap::new(),
            floats: HashMap::new(),
            strings: HashMap::new(),
            point2s: HashMap::new(),
            point3s: HashMap::new(),
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
        };
    }
}

fn split_variable_type_name(token: String) -> (String, String) {
    let parts = token.split_whitespace().collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);

    return (String::from(parts[0]), String::from(parts[1]));
}

fn fetch_variable_value(value: &Value) -> Vec<String> {
    return value
        .as_array()
        .unwrap()
        .into_iter()
        .map(|v| json_value_to_string(v.clone()))
        .collect();
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

fn convert_string<T: FromStr>(string_vec: &Vec<String>) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    return string_vec
        .into_iter()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>();
}

impl ParameterDict {
    pub fn build_from_vec(array: &[Value]) -> Self {
        let mut integers = HashMap::<String, Vec<i32>>::new();
        let mut floats = HashMap::<String, Vec<Float>>::new();
        let mut strings = HashMap::<String, String>::new();
        let mut point2s = HashMap::<String, Vec<Point2f>>::new();
        let mut point3s = HashMap::<String, Vec<Point3f>>::new();

        for idx in (0..array.len()).step_by(2) {
            let token = trim_quote(json_value_to_string(array[idx].clone()));
            let (variable_type, variable_name) = split_variable_type_name(token);
            let variable_values = fetch_variable_value(&array[idx + 1]);

            match variable_type.as_str() {
                "string" => {
                    assert_eq!(variable_values.len(), 1);
                    strings.insert(variable_name, variable_values[0].clone());
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
                _ => {
                    panic!("unknown variable type: `{}`", variable_type);
                }
            }
        }

        return ParameterDict {
            integers,
            floats,
            strings,
            point2s,
            point3s,
        };
    }

    pub fn insert_integer(&mut self, name: String, value: Vec<i32>) {
        self.integers.insert(name, value);
    }

    pub fn insert_float(&mut self, name: String, value: Vec<Float>) {
        self.floats.insert(name, value);
    }

    pub fn insert_string(&mut self, name: String, value: String) {
        self.strings.insert(name, value);
    }

    pub fn get_one_float(&self, key: &str, default: Option<Float>) -> Float {
        match self.floats.get(key) {
            None => match default {
                None => {
                    panic!("found no key with name `{}`", key);
                }
                Some(val_default) => {
                    return val_default;
                }
            },
            Some(val) => {
                if val.len() != 1 {
                    panic!("array length is larger than 1");
                }
                return val[0];
            }
        };
    }

    pub fn get_one_integer(&self, key: &str, default: Option<i32>) -> i32 {
        match self.integers.get(key) {
            None => match default {
                None => {
                    panic!("found no key with name `{}`", key);
                }
                Some(val_default) => {
                    return val_default;
                }
            },
            Some(val) => {
                if val.len() != 1 {
                    panic!("array length is larger than 1");
                }
                return val[0];
            }
        };
    }

    pub fn get_integer_array(&self, key: &str) -> Vec<i32> {
        return match self.integers.get(key) {
            None => {
                panic!("found no key with name `{}`", key);
            }
            Some(val) => val.clone(),
        };
    }

    pub fn get_point3_array(&self, key: &str) -> Vec<Point3f> {
        return match self.point3s.get(key) {
            None => {
                panic!("found no key with name `{}`", key);
            }
            Some(val) => val.clone(),
        };
    }

    pub fn get_string(&self, key: &str) -> String {
        return match self.strings.get(key) {
            None => {
                panic!("found no key with name `{}`", key);
            }
            Some(val) => val.clone(),
        };
    }

    pub fn display(&self) {}
}

impl Display for ParameterDict {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "strings: {}\n", self.strings.len()).expect("error");
        write!(f, "integers: {}\n", self.integers.len()).expect("error");
        write!(f, "floats: {}\n", self.floats.len()).expect("error");
        write!(f, "point2s: {}\n", self.point2s.len()).expect("error");
        write!(f, "point3s: {}\n", self.point3s.len()).expect("error");

        return Ok(());
    }
}
