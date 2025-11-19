pub mod window_controller;
#[cfg(target_os = "macos")]
pub mod mac_window_controller;

use tauri::{Runtime, Window};
use window_controller::WindowController;

#[cfg(target_os = "macos")]
use mac_window_controller::MacWindowController;
#[cfg(not(target_os = "macos"))]
use window_controller::NoOpWindowController;

#[tauri::command]
fn resize_window<R: Runtime>(window: Window<R>, width: f64, height: f64) -> Result<(), String> {
    // 1. Resize the window physically
    window.set_size(tauri::Size::Logical(tauri::LogicalSize { width, height }))
        .map_err(|e| e.to_string())?;

    // 2. Set aspect ratio constraint
    #[cfg(target_os = "macos")]
    {
        let controller = MacWindowController::new(window);
        controller.set_aspect_ratio(width, height)?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        let controller = NoOpWindowController;
        <NoOpWindowController as WindowController<R>>::set_aspect_ratio(&controller, width, height)?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![resize_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
