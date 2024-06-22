use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    id: u32,
    title: String,
    password: String,
}

pub(crate) fn add_credential(title: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from("assets");
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
    let mut path = PathBuf::from("assets");
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

pub(crate) fn delete_credential(id: u32) -> Result<(), Box<dyn std::error::Error>>{
    let mut path: PathBuf = PathBuf::from("assets");
    path.push("credentials.json");

    if let Some(parent) = path.parent() {
     println!("Создается директория: {:?}" , parent);
     match fs::create_dir_all(parent) {
        Ok(_)=>println!("Директория создана успешно"),
        Err(e)=>println!("Не получилось создать  директорию:{}", e),

     }
}
 
 let mut credentials: Vec<Credentials> = if path.exists() {
    let data = fs::read_to_string(&path)?;
    serde_json::from_str(&data)?
 }
 else {
  return Ok(());
 };


 credentials.retain(|cred| cred.id  != id );

 let json_data = serde_json::to_string_pretty(&credentials)?;

 println!("сериализованные данные json:{}", json_data);

let mut file = match fs::File::create(&path) {
    Ok(file) => file,
    Err(e) => {
        println!("не удалось создать файл:{}", e);
        return Err(Box::new(e));
    }
};

match file.write_all(json_data.as_bytes()) {
    Ok(_) => println!("Учетка с id {} удалена", id),
    Err(e) => println!("Не удалось удалить учетку:{}", e),
}

Ok(())


}

pub(crate) fn update_credential(id:u32, title: &str, password:&str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path: PathBuf = PathBuf::from("assets");
    path.push("credentials.json");

    if let Some(parent) = path.parent() {
     println!("Создается директория: {:?}" , parent);
     match fs::create_dir_all(parent) {
        Ok(_)=>println!("Директория создана успешно"),
        Err(e)=>println!("Не получилось создать  директорию:{}", e),

     }
}
 
 let mut credentials: Vec<Credentials> = if path.exists() {
    let data = fs::read_to_string(&path)?;
    serde_json::from_str(&data)?
 }
 else {
  return Ok(());
 };


 if let Some(credential) = credentials.iter_mut().find(|cred| cred.id == id) {
    credential.title = title.to_string();
    credential.password = password.to_string();
 }

 let json_data = serde_json::to_string_pretty(&credentials)?;

 println!("сериализованные данные json:{}", json_data);

let mut file = match fs::File::create(&path) {
    Ok(file) => file,
    Err(e) => {
        println!("не удалось создать файл:{}", e);
        return Err(Box::new(e));
    }
};

match file.write_all(json_data.as_bytes()) {
    Ok(_) => println!("Учетка с id {} удалена", id),
    Err(e) => println!("Не удалось удалить учетку:{}", e),
}

Ok(())




}