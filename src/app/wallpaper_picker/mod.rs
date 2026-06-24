use slint::{ComponentHandle, Image, Model, ModelRc, SharedString, VecModel};
use crate::config;
use crate::wallpaper;

pub struct WallpaperPickerController;

impl WallpaperPickerController {
    pub fn connect(window: &crate::WallpaperPickerWindowSpell) {
        let adapter = window.global::<crate::WallpaperPickerAdapter>();

        let weak = window.as_weak();

        adapter.on_escape_pressed({ 
            move || {
                std::process::exit(0);               
            }
        });
    }
}
