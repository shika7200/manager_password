// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod credential;

use std::fs::{ OpenOptions};
use std::io:: { Read, Seek, SeekFrom , Write};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
struct Password {
    password: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
struct Passwords {
    passwords: Vec<Password>
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn save_password(password: &str, title: &str) {
    // Пытаемся открыть файл для чтения и записи
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Создаем файл, если он не существует
        .open("passwords.json") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Не получилось открыть файл: {}", err);
            return; // В случае ошибки завершаем функцию
        }
    };

    // Пытаемся прочитать содержимое файла
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        eprintln!("Failed to read file: {}", err);
        return; // В случае ошибки завершаем функцию
    }

    // Если файл пустой или содержит некорректные данные, создаем новую структуру
    let mut passwords_data: Passwords = if contents.trim().is_empty() {
        Passwords { passwords: vec![] }
    } else {
        match serde_json::from_str(&contents) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Неудачная попытка распарсить JSON: {}", err);
                return; // В случае ошибки завершаем функцию
            }
        }
    };

    // Добавляем новый пароль к существующим
    let new_password = Password {
        password: password.to_string(),
        title: title.to_string(),
    };
    passwords_data.passwords.push(new_password);

    // Пытаемся записать структуру в файл
    if let Err(err) = file.seek(SeekFrom::Start(0)) {
        eprintln!("Failed to seek file: {}", err);
        return; // В случае ошибки завершаем функцию
    }
    if let Err(err) = file.set_len(0) {
        eprintln!("Failed to truncate file: {}", err);
        return; // В случае ошибки завершаем функцию
    }
    if let Err(err) = file.write_all(serde_json::to_string_pretty(&passwords_data).unwrap().as_bytes()) {
        eprintln!("Failed to write to file: {}", err);
        return; // В случае ошибки завершаем функцию
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
