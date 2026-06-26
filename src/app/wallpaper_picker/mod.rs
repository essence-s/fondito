pub mod picker;
pub mod services;

use slint::ComponentHandle;

pub struct WallpaperPickerController;

impl WallpaperPickerController {
    pub fn connect(window: &crate::WallpaperPickerWindowSpell) {
        let adapter = window.global::<crate::WallpaperPickerAdapter>();

        adapter.on_escape_pressed({
            move || {
                std::process::exit(0);
            }
        });
    }
}
