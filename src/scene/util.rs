use crate::pbrt::*;

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
