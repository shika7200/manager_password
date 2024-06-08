use serde::{Serialize, Deserialize};
use std::{fs};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    id: u32,
    title: String,
    password: String,
}

pub(crate) fn add_credential(title: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from("src-tauri");
    path.push("credentials.json");

    println!("Adding credential: {} - {}", title, password); // Отладка

    // Создание директории, если она не существует
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut credentials: Vec<Credentials> = if path.exists() {
        let data = fs::read_to_string(&path)?;
        serde_json::from_str(&data)?
    } else {
        Vec::new()
    };

    let id = credentials.len() as u32 + 1;
    credentials.push(Credentials {
        id,
        title: title.to_string(),
        password: password.to_string(),
    });

    let json_data = serde_json::to_string_pretty(&credentials)?;
    println!("Serialized JSON data: {}", json_data); // Отладка

    let write_result = fs::write(&path, json_data);
    match write_result {
        Ok(_) => println!("Credentials successfully saved to {:?}", path),
        Err(e) => println!("Failed to save credentials: {}", e),
    }

    Ok(())
}

pub(crate) fn get_credential() -> Result<Vec<Credentials>, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from("src-tauri");
    path.push("credentials.json");

    if path.exists() {
        let data = fs::read_to_string(&path)?;
        let credentials: Vec<Credentials> = serde_json::from_str(&data)?;

        println!("Credentials read from {:?}", path);
        println!("Current credentials: {:?}", credentials);

        Ok(credentials)
    } else {
        Ok(Vec::new())
    }
}
