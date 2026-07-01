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

        let weak = window.as_weak();
        slint::Timer::single_shot(std::time::Duration::from_millis(40), move || {
            if let Some(win) = weak.upgrade() {
                let adapter = win.global::<crate::WallpaperPickerAdapter>();
                adapter.set_render_trigger(1);
            }
        });
    }
}
