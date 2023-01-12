use std::fs::File;
use std::io::{Read};
use std::{str, fs};

use base64::{engine::general_purpose, Engine as _};
use directories::ProjectDirs;

fn get_path(name: String) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("im", "khad",  "drawbridge") {
        let mut tmp = proj_dirs.data_local_dir().to_path_buf();
        tmp.push(format!("drawbridge_{}.bin", name));
        return Ok(tmp);
    }
    return Err("Could not get project directories.".into());
}

pub fn save(name: String, content: String) -> Result<(), Box<dyn std::error::Error>> {
    let file_name= get_path(name).unwrap();

    let mut dir_path = file_name.clone();
    dir_path.pop();
    fs::create_dir_all(dir_path)?;

    fs::write(file_name, general_purpose::STANDARD.encode(&content).as_bytes())?;
    return Ok(());
}

pub fn load(name: String) -> Result<String, Box<dyn std::error::Error>> {
    let file_name= get_path(name).unwrap();

    let mut file = File::open(file_name)?;
    let mut content: String = String::new();

    file.read_to_string(&mut content)?;
    let decoded = &general_purpose::STANDARD.decode(content.as_bytes()).unwrap();
    let content = str::from_utf8(decoded).unwrap();

    return Ok(content.to_string());
}