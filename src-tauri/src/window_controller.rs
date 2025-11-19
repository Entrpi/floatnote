use tauri::Runtime;

pub trait WindowController<R: Runtime> {
    fn set_aspect_ratio(&self, width: f64, height: f64) -> Result<(), String>;
}

pub struct NoOpWindowController;

impl<R: Runtime> WindowController<R> for NoOpWindowController {
    fn set_aspect_ratio(&self, _width: f64, _height: f64) -> Result<(), String> {
        // No-op for platforms that don't support aspect ratio locking (e.g., Linux Wayland fallback)
        Ok(())
    }
}
