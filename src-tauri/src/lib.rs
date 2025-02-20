// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lpg;

#[tauri::command]
fn greet(name: &str) -> String {
    let params = lpg::crop_tool::CropParams {
        input: "./input".to_string(),
        output: "./output".to_string(),
        template: "./templates".to_string(),
        do_generate_posters: true,
        do_generate_paintings: true,
    };
    lpg::crop_tool::generate(&params);
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
