use crate::pbrt::*;

pub fn json_value_to_usize(value: Value) -> usize {
    return serde_json::from_value(value).unwrap();
}

pub fn json_value_to_string(value: Value) -> String {
    return serde_json::from_value(value).unwrap();
}

pub fn json_value_to_floats(value: Value) -> Vec<Float> {
    return value
        .as_array()
        .unwrap()
        .into_iter()
        .map(|x| json_value_to_string(x.clone()).parse::<Float>().unwrap())
        .collect();
}

pub fn json_values_to_floats(values: &[Value]) -> Vec<Float> {
    return values
        .into_iter()
        .map(|v| json_value_to_string(v.clone()).parse::<Float>().unwrap())
        .collect();
}

pub fn trim_quote(token: String) -> String {
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

pub fn get_basename(path: &str) -> String {
    return match path.rfind("/") {
        None => {
            panic!("couldn't find `/` from {}", path);
        }
        Some(pos) => path.chars().skip(pos + 1).collect(),
    };
}

pub fn get_dirname(path: &str) -> String {
    return match path.rfind("/") {
        None => {
            panic!("couldn't find `/` from {}", path);
        }
        Some(pos) => path.chars().take(pos).collect(),
    };
}

pub fn get_extension(path: &str) -> String {
    match path.rfind(".") {
        None => {
            panic!("couldn't find `.` from {}", path);
        }
        Some(pos) => {
            return path.chars().skip(pos + 1).take(usize::MAX).collect();
        }
    };
}

pub fn change_extension(path: &str, extension: &str) -> String {
    match path.rfind(".") {
        None => {
            panic!("couldn't find `.` from {}", path);
        }
        Some(pos) => {
            let mut filename: String = path.chars().take(pos).collect();

            filename.push('.');
            filename.push_str(extension);

            return filename;
        }
    };
}
