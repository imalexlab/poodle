#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;

use sha256::try_digest;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn checksum(path: &str, sum: &str) -> String {
    println!("path {}, sum {}", path, sum);
    let input = Path::new(path);

    let path_exist = input.is_file();

    if !path_exist {
        format!("❌ File does not exist, please verify the path ❌")
    } else {
        let val = try_digest(input);
        match val {
            Ok(v) => {
                if v == sum.to_string() {
                    format!("🚀 File is correct, checksums are the same 🚀")
                } else {
                    format!("⚠️ Invalid checksum ⚠️")
                }
            }
            Err(e) => {
                format!("❌ Error while getting the file checksum ❌, {}", e)
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![checksum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
