use crate::pbrt::*;

pub struct ParameterDict {
    integers: HashMap<String, Vec<i32>>,
    floats: HashMap<String, Vec<Float>>,
    strings: HashMap<String, String>,
}

impl Default for ParameterDict {
    fn default() -> Self {
        return ParameterDict {
            integers: HashMap::new(),
            floats: HashMap::new(),
            strings: HashMap::new(),
        };
    }
}

fn split_variable_type_name(token: String) -> (String, String) {
    let parts = token.split_whitespace().collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);

    return (String::from(parts[0]), String::from(parts[1]));
}

fn fetch_variable_value(value: &Value) -> Vec<String> {
    //let value_vec = (value.as_array().unwrap()).clone();
    return value
        .as_array()
        .unwrap()
        .into_iter()
        .map(|v| json_value_to_string(v.clone()))
        .collect();
}

impl ParameterDict {
    pub fn build_from_vec(array: &[Value]) -> Self {
        let mut _integers = HashMap::<String, Vec<i32>>::new();
        let mut _floats = HashMap::<String, Vec<Float>>::new();
        let mut _strings = HashMap::<String, String>::new();

        for idx in (0..array.len()).step_by(2) {
            let token = trim_quote(json_value_to_string(array[idx].clone()));
            let (variable_type, variable_name) = split_variable_type_name(token);
            let variable_values = fetch_variable_value(&array[idx + 1]);

            match variable_type.as_str() {
                "string" => {
                    assert_eq!(variable_values.len(), 1);
                    _strings.insert(variable_name, variable_values[0].clone());
                }
                "integer" => {
                    let values = variable_values
                        .into_iter()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    _integers.insert(variable_name, values);
                }
                "float" => {
                    let values = variable_values
                        .into_iter()
                        .map(|x| x.parse::<Float>().unwrap())
                        .collect::<Vec<Float>>();
                    _floats.insert(variable_name, values);
                }
                _ => {
                    println!("unkown variable type: `{}`", variable_type);
                }
            }
        }

        return ParameterDict {
            integers: _integers,
            floats: _floats,
            strings: _strings,
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

    pub fn get_one_float_with_default(&self, key: &str, default: Float) -> Float {
        return match self.floats.get(key) {
            None => default,
            Some(val) => {
                assert_eq!(val.len(), 1);
                val[0]
            }
        };
    }

    pub fn get_one_integer_with_default(&self, key: &str, default: i32) -> i32 {
        return match self.integers.get(key) {
            None => default,
            Some(val) => {
                assert_eq!(val.len(), 1);
                val[0]
            }
        };
    }

    pub fn get_one_integer_or_panic(&self, key: &str) -> i32 {
        return match self.integers.get(key) {
            None => {
                panic!("found no key with name `{}`", key);
            }
            Some(val) => {
                assert_eq!(val.len(), 1);
                val[0]
            }
        };
    }

    pub fn get_string_or_panic(&self, key: &str) -> String {
        return match self.strings.get(key) {
            None => {
                panic!("found no key with name `{}`", key);
            }
            Some(val) => val.clone(),
        };
    }

    pub fn display(&self) {
        println!("strings: {}", self.strings.len());
        for (k, v) in &self.strings {
            println!("{} -> {}", k, v);
        }
        println!();

        println!("integers: {}", self.integers.len());
        for (k, values) in &self.integers {
            print!("{} -> ", k);
            print!("[ ");
            for v in values {
                print!("{} ", v);
            }
            print!("]\n");
        }
        println!();

        println!("floats: {}", self.floats.len());
        for (k, values) in &self.floats {
            print!("{} -> ", k);
            print!("[ ");
            for v in values {
                print!("{} ", v);
            }
            print!("]\n");
        }
        println!();
    }
}
