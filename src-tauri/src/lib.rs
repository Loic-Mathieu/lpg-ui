use std::path::{Path, PathBuf};
use tauri::Manager;
use tauri::path::BaseDirectory;
use crate::lpg::crop_tool::Modes::Posters;
use crate::lpg::crop_tool::CropParams;
use crate::lpg::crop_tool::TEMPLATE_DIR;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lpg;

fn load_resource(handle: &tauri::AppHandle, resource: &str) -> tauri::Result<PathBuf> {
    let path = Path::new("resources").join(resource);
    handle.path().resolve(path, BaseDirectory::Resource)
}

#[tauri::command]
async fn greet(handle: tauri::AppHandle, name: String) -> String {
    let template_dir = load_resource(&handle, TEMPLATE_DIR).unwrap();

    let params = CropParams{
        input: "./input".to_string(),
        output: "./output".to_string(),
        template_dir,
        modes: vec![Posters],
    };

    println!("Generating picture...");
    lpg::crop_tool::generate(&params).await;
    println!("Generation complete !");

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
