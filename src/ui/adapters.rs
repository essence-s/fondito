use crate::app::wallpaper_picker::WallpaperPickerController;
use crate::app::wallpaper_picker::picker::PickerController;

pub fn connect_all(window: &crate::WallpaperPickerWindowSpell) {
    PickerController::connect(window);
    WallpaperPickerController::connect(window);
}
