pub mod window_controller;
#[cfg(target_os = "macos")]
pub mod mac_window_controller;

use tauri::{Runtime, Window, AppHandle, Emitter, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder, CheckMenuItemBuilder};
use window_controller::WindowController;

#[cfg(target_os = "macos")]
use mac_window_controller::MacWindowController;
#[cfg(not(target_os = "macos"))]
use window_controller::NoOpWindowController;

#[tauri::command]
fn resize_window<R: Runtime>(window: Window<R>, width: f64, height: f64) -> Result<(), String> {
    let scale_factor = window.scale_factor().map_err(|e| e.to_string())?;
    
    // Get Screen Dimensions
    let monitor = window.current_monitor().map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    let screen_size = monitor.size().to_logical::<f64>(scale_factor);

    // Calculate Max Dimensions (80% of Screen)
    let max_w = screen_size.width * 0.8;
    let max_h = screen_size.height * 0.8;

    let mut target_w = width;
    let mut target_h = height;

    // Scale down if too big
    if target_w > max_w || target_h > max_h {
        let ratio_w = max_w / target_w;
        let ratio_h = max_h / target_h;
        let scale = f64::min(ratio_w, ratio_h);
        
        target_w *= scale;
        target_h *= scale;
    }

    // Apply Size
    window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: target_w, height: target_h }))
        .map_err(|e| e.to_string())?;

    // Apply Aspect Ratio Lock
    #[cfg(target_os = "macos")]
    {
        let controller = MacWindowController::new(window);
        controller.set_aspect_ratio(target_w, target_h)?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        let controller = NoOpWindowController;
        <NoOpWindowController as WindowController<R>>::set_aspect_ratio(&controller, target_w, target_h)?;
    }

    Ok(())
}

#[tauri::command]
fn zoom_window<R: Runtime>(window: Window<R>, factor: f64) -> Result<(), String> {
    let scale_factor = window.scale_factor().map_err(|e| e.to_string())?;
    let size = window.inner_size().map_err(|e| e.to_string())?
        .to_logical::<f64>(scale_factor);

    let new_width = size.width * factor;
    let new_height = size.height * factor;

    // Minimum constraint (50px)
    if new_width < 50.0 || new_height < 50.0 {
        return Ok(());
    }

    // Maximum constraint (Screen Size)
    let monitor = window.current_monitor().map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    let screen_size = monitor.size().to_logical::<f64>(scale_factor);

    // If growing (factor > 1.0) and we exceed screen size, stop.
    if factor > 1.0 {
        if new_width > screen_size.width || new_height > screen_size.height {
            return Ok(());
        }
    }

    window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: new_width,
        height: new_height,
    })).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn show_context_menu<R: Runtime>(app: AppHandle<R>, window: Window<R>, is_interactive: bool) -> Result<(), String> {
    let opacity_100 = MenuItemBuilder::new("100%").id("opacity_1.0").build(&app).map_err(|e| e.to_string())?;
    let opacity_75 = MenuItemBuilder::new("75%").id("opacity_0.75").build(&app).map_err(|e| e.to_string())?;
    let opacity_50 = MenuItemBuilder::new("50%").id("opacity_0.5").build(&app).map_err(|e| e.to_string())?;
    let opacity_25 = MenuItemBuilder::new("25%").id("opacity_0.25").build(&app).map_err(|e| e.to_string())?;

    let opacity_menu = SubmenuBuilder::new(&app, "Opacity")
        .items(&[&opacity_100, &opacity_75, &opacity_50, &opacity_25])
        .build().map_err(|e| e.to_string())?;

    let is_pinned = window.is_always_on_top().unwrap_or(false);
    let pin_item = CheckMenuItemBuilder::new("Always on Top")
        .id("pin_toggle")
        .checked(is_pinned)
        .build(&app).map_err(|e| e.to_string())?;

    let interactive_item = CheckMenuItemBuilder::new("Interactive")
        .id("toggle_interaction")
        .checked(is_interactive)
        .build(&app).map_err(|e| e.to_string())?;

    let close_item = MenuItemBuilder::new("Close").id("close").build(&app).map_err(|e| e.to_string())?;

    let menu = MenuBuilder::new(&app)
        .items(&[&interactive_item, &pin_item, &opacity_menu, &close_item])
        .build().map_err(|e| e.to_string())?;

    window.popup_menu(&menu).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![resize_window, zoom_window, show_context_menu])
        .on_menu_event(|app, event| {
            let id = event.id();
            if let Some(window) = app.get_webview_window("main") {
                if id == "close" {
                    app.exit(0);
                } else if id == "pin_toggle" {
                     let current = window.is_always_on_top().unwrap_or(false);
                     let _ = window.set_always_on_top(!current);
                } else if id == "toggle_interaction" {
                     let _ = window.emit("toggle_interaction", ());
                } else if id.as_ref().starts_with("opacity_") {
                    if let Ok(val) = id.as_ref().trim_start_matches("opacity_").parse::<f64>() {
                        let _ = window.emit("set_opacity", val);
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
