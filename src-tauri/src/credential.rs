use serde::{Serialize, Deserialize};
use serde_json :: {Value, json};
use std::{env,  fs};
use std:: path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Credentials {
    title: String,
    password: String,
}


fn add_credential (title: &str, password: &str)-> Result<() , Box<dyn std::error::Error>>{
    let app_dir = env::var("TAURI_APP_DIR")?;
    let mut path = PathBuf::from(app_dir);
    path.push("public/credentials.json");

    let mut credentials:Vec<Credentials> = if path.exists() {
        let data = fs::read_to_string(&path)?;
        serde_json::from_str(&data)?
    }
    else {
        Vec:: new()
    };
    credentials.push(Credentials{
        title: title.to_string(),
        password: password.to_string(),
    });
    let json_data = serde_json::to_string_pretty(&credentials)?;
    fs::write(path, json_data)?;
    Ok(())
}


fn get_credential () -> Result<Vec<Credentials>, Box<dyn std::error::Error>>{
    let app_dir = env::var("TAURI_APP_DIR")?;
    let mut path = PathBuf::from(app_dir);
    path.push("public/credentials.json");

    if path.exists() {
        let data = fs::read_to_string(&path)?;
        let credentials: Vec<Credentials> = serde_json::from_str(&data)?;
        Ok(credentials)
    }
    else {
        Ok(Vec::new())
    }
}   
