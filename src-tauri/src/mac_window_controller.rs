#[cfg(target_os = "macos")]
use crate::window_controller::WindowController;
#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
#[cfg(target_os = "macos")]
use cocoa::base::id;
#[cfg(target_os = "macos")]
use cocoa::foundation::NSSize;
#[cfg(target_os = "macos")]
use objc::runtime::{Object, Sel};
#[cfg(target_os = "macos")]
use tauri::{Runtime, Window};

#[cfg(target_os = "macos")]
pub struct MacWindowController<R: Runtime> {
    window: Window<R>,
}

#[cfg(target_os = "macos")]
impl<R: Runtime> MacWindowController<R> {
    pub fn new(window: Window<R>) -> Self {
        Self { window }
    }
}

#[cfg(target_os = "macos")]
impl<R: Runtime> WindowController<R> for MacWindowController<R> {
    fn set_aspect_ratio(&self, width: f64, height: f64) -> Result<(), String> {
        let ns_window: id = self.window.ns_window().map_err(|e| e.to_string())? as id;
        unsafe {
            let size = NSSize::new(width, height);
            let _: () = msg_send![ns_window, setContentAspectRatio: size];
        }
        Ok(())
    }
}
