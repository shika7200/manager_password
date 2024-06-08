#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod credential;
use credential::{get_credential, add_credential, update_credential, delete_credential, Credentials};

#[tauri::command]
fn add_credential_command(title: String, login: String, password: String) -> Result<(), String> {
    add_credential(&title, &login, &password).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_credential_command() -> Result<Vec<Credentials>, String> {
    get_credential().map_err(|e| e.to_string())
}

#[tauri::command]
fn update_credential_command(id: u32, title: String, login: String, password: String) -> Result<(), String> {
    update_credential(id, &title, &login, &password).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_credential_command(id: u32) -> Result<(), String> {
    delete_credential(id).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_credential_command,
            get_credential_command,
            update_credential_command,
            delete_credential_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
