use crate::lpg::crop_tool::{ListedFile, TEMPLATE_DIR};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::{to_value};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager, Result};
use tauri_plugin_store::StoreExt;
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
async fn generate(app: AppHandle, package_name: String, files: Vec<ListedFile>) -> Result<Response> {
    app.emit("loading", true)?;

    // TODO handle dir existence
    let template_dir = load_resource(&app, TEMPLATE_DIR)?;
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().to_owned();

    // Load lpg.settings
    let settings = app.store("settings.json").unwrap();
    let lpg_value = settings.get("lpg").expect("lpg settings is missing");
    let lpg_settings: lpg::settings::Settings = serde_json::from_value(lpg_value)?;
    // TODO validate value
    let output_dir = PathBuf::from(lpg_settings.output);
    println!("Mount > {}", output_dir.display());

    // Generation
    println!("Generating pictures...");
    lpg::crop_tool::generate(files, temp_path.clone(), template_dir.clone()).await;
    println!("Generation complete !");

    // Packaging
    println!("Packaging...");
    lpg::package_tool::create(temp_path.clone(), output_dir.clone(), &package_name).await;
    println!("Packaging complete !");

    println!("Temp file {} should be deleted.", temp_path.display());
    temp_dir.close()?;

    app.emit("loading", false)?;
    Ok(Response {
        message: format!("Package {} was successfully generated !", package_name),
    })
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub plugin_path: String,
}

#[tauri::command]
async fn load(app: AppHandle, package_name: String) -> Result<Response> {
    app.emit("loading", true)?;

    // Load lpg.settings
    let settings = app.store("settings.json").unwrap();
    let lpg_value = settings.get("lpg").expect("lpg settings is missing");
    let lpg_settings: lpg::settings::Settings = serde_json::from_value(lpg_value)?;
    // TODO validate value
    let output_dir = PathBuf::from(lpg_settings.output);
    println!("Mount > {}", output_dir.display());

    let global_value = settings.get("global").expect("global settings is missing");
    let global_settings: Settings = serde_json::from_value(global_value)?;
    // TODO validate value
    let plugins_dir = PathBuf::from(global_settings.plugin_path);

    println!("Loading package...");
    lpg::package_tool::load(output_dir, &package_name, plugins_dir).await;
    println!("Package loaded !");

    app.emit("loading", false)?;
    Ok(Response {
        message: format!("Package {} was successfully loaded !", package_name),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let settings = app.store("settings.json")?;

            // Set default lpg output
            if !settings.has("lpg") {
                let default_lpg = to_value(lpg::settings::Settings {
                    output: "output".to_string()
                })?;
                settings.set("lpg", default_lpg);
                println!("Default lpg setting set !");
            }

            // settings.close_resource();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate, load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
