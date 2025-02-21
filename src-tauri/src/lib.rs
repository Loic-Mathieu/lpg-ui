use std::env::temp_dir;
use std::path::PathBuf;
use tauri::Manager;
use tauri::path::BaseDirectory;
use tempfile::TempDir;
use crate::lpg::crop_tool::{TEMPLATE_DIR, CropParams, Modes};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod lpg;

fn load_resource(handle: &tauri::AppHandle, resource: &str) -> tauri::Result<PathBuf> {
    let path = PathBuf::from("resources").join(resource);
    handle.path().resolve(path, BaseDirectory::Resource)
}

#[tauri::command]
async fn greet(handle: tauri::AppHandle, name: String) -> String {
    // TODO handle dir existence
    let template_dir = load_resource(&handle, TEMPLATE_DIR).unwrap();
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_owned();
    let output_dir = PathBuf::from("output"); // TODO this should be parameterizable

    // Generation
    println!("Generating pictures...");
    let params = CropParams{
        input: "./input".to_string(),
        output_dir: temp_path.clone(),
        template_dir,
        modes: vec![Modes::Posters],
    };

    lpg::crop_tool::generate(&params).await;
    println!("Generation complete !");

    // Packaging
    println!("Packaging...");
    lpg::package_tool::create(temp_path.clone(), output_dir, &name).await;
    println!("Packaging complete !");

    println!("Temp file {} should be deleted.", temp_path.display());
    temp_dir.close().unwrap();

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
