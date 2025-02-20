use crate::lpg::crop_tool::Modes::Posters;
use crate::lpg::crop_tool::CropParams;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lpg;

#[tauri::command]
async fn greet(name: String) -> String {
    let params = CropParams{
        input: "./input".to_string(),
        output: "./output".to_string(),
        template: "./templates".to_string(),
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
