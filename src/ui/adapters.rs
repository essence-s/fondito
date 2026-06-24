use crate::app::wallpaper_picker::WallpaperPickerController;

pub fn connect_all(window: &crate::WallpaperPickerWindowSpell) {
    WallpaperPickerController::connect(window);
}
