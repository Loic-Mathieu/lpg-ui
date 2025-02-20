use tauri::Manager;
use tauri::path::BaseDirectory;
use crate::lpg::crop_tool::Modes::Posters;
use crate::lpg::crop_tool::CropParams;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lpg;

#[tauri::command]
async fn greet(handle: tauri::AppHandle, name: String) -> String {
    let p = handle.path().resolve("resources\\templates\\posters_template.png", BaseDirectory::Resource).unwrap();
    let p2 = handle.path().resolve("resources\\templates\\posters_template.png", BaseDirectory::Resource).unwrap();
    let p3 = handle.path().resolve("resources\\templates", BaseDirectory::Resource).unwrap();

    println!("Hello, {:?}!", p3);

    let params = CropParams{
        input: "./input".to_string(),
        output: "./output".to_string(),
        // template: "./templates".to_string(),
        poster_template: p,
        painting_template: p2,
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
