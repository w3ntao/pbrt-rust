use crate::pbrt::*;

pub fn json_value_to_usize(value: Value) -> usize {
    serde_json::from_value(value).unwrap()
}

pub fn json_value_to_string(value: Value) -> String {
    serde_json::from_value(value).unwrap()
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

pub fn get_folder_potion(path: &str) -> String {
    return match path.rfind("/") {
        None => {
            panic!("couldn't find `/` from {}", path);
        }
        Some(pos) => path.chars().take(pos).collect(),
    };
}
