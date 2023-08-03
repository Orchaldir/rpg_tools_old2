use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write<T: Serialize>(object: &T, path: &Path) {
    let mut file = File::create(path).unwrap();

    let s = serde_yaml::to_string(object).unwrap();

    file.write_all(s.as_bytes()).unwrap();
}
