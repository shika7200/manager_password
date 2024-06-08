// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod credential;
use credential::{get_credential, add_credential, Credentials};


#[tauri::command]
fn add_credential_command (title:String, password:String) -> Result<(), String> {
    add_credential(&title, &password)
    .map_err(|e| e.to_string())
}
#[tauri::command]
fn get_credential_command() -> Result<Vec<Credentials> ,String > {
    get_credential()
    .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_credential_command, get_credential_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


