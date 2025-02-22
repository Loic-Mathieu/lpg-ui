use crate::lpg::crop_tool::{CropParams, TEMPLATE_DIR};
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager, Result};
use tempfile::TempDir;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod lpg;

fn load_resource(handle: &AppHandle, resource: &str) -> Result<PathBuf> {
    let path = PathBuf::from("resources").join(resource);
    handle.path().resolve(path, BaseDirectory::Resource)
}

#[derive(serde::Serialize, Clone)]
struct Response {
    message: String,
}

#[tauri::command]
async fn generate(app: AppHandle, package_name: String, modes: Vec<String>) -> Result<Response> {
    app.emit("loading", true)?;

    // TODO handle dir existence
    let template_dir = load_resource(&app, TEMPLATE_DIR)?;
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().to_owned();
    let output_dir = PathBuf::from("output"); // TODO this should be parameterizable

    // Generation
    println!("Generating pictures...");
    let params = CropParams {
        input: "./input".to_string(),
        output_dir: temp_path.clone(),
        template_dir,
        modes: modes.iter().map(|m| m.to_uppercase().parse().unwrap()).collect(),
    };

    lpg::crop_tool::generate(&params).await;
    println!("Generation complete !");

    // Packaging
    println!("Packaging...");
    lpg::package_tool::create(temp_path.clone(), output_dir, &package_name).await;
    println!("Packaging complete !");

    println!("Temp file {} should be deleted.", temp_path.display());
    temp_dir.close()?;

    app.emit("loading", false)?;
    Ok(Response{
        message: format!("Package {} was successfully generated !", package_name),
    })
}

#[tauri::command]
async fn load(name: String) {
    // TODO implement
    let output_dir = PathBuf::from("output"); // TODO this should be parameterizable
    lpg::package_tool::load(output_dir, &name).await;
}

#[tauri::command]
async fn list_files(name: String) {
    // TODO implement
    let output_dir = PathBuf::from("output"); // TODO this should be parameterizable
    lpg::package_tool::read_metadata(output_dir, &name).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate, load, list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
