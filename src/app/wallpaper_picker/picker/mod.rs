use slint::{ComponentHandle, Model};

pub struct PickerController;

impl PickerController {
    pub fn connect(window: &crate::WallpaperPickerWindowSpell) {
        let adapter = window.global::<crate::PickerAdapter>();
        let weak = window.as_weak();

        let empty_model =
            slint::ModelRc::new(slint::VecModel::from(Vec::<crate::WallpaperItem>::new()));
        adapter.set_wallpaper_model(empty_model.clone());
        adapter.set_current_index(0);

        adapter.on_navigate_left({
            let weak = weak.clone();
            move || {
                if let Some(w) = weak.upgrade() {
                    let a = w.global::<crate::PickerAdapter>();
                    let idx = a.get_current_index();
                    if idx > 0 {
                        a.set_current_index(idx - 1);
                    }
                }
            }
        });

        adapter.on_navigate_right({
            let model = empty_model.clone();
            let weak = weak.clone();
            move || {
                if let Some(w) = weak.upgrade() {
                    let a = w.global::<crate::PickerAdapter>();
                    let len = model.row_count() as i32;
                    let idx = a.get_current_index();
                    if idx < len.saturating_sub(1).max(0) {
                        a.set_current_index(idx + 1);
                    }
                }
            }
        });

        adapter.on_apply_current({
            let weak = weak.clone();
            move || {
                if let Some(w) = weak.upgrade() {
                    let a = w.global::<crate::PickerAdapter>();
                    let idx = a.get_current_index();
                    let model = a.get_wallpaper_model();
                    if let Some(item) = model.row_data(idx as usize) {
                        log::info!(
                            "Apply wallpaper: {} (video: {})",
                            item.file_name,
                            item.is_video
                        );
                    }
                }
            }
        });

        adapter.on_card_clicked({
            let weak = weak.clone();
            move |idx| {
                if let Some(w) = weak.upgrade() {
                    let a = w.global::<crate::PickerAdapter>();
                    a.set_current_index(idx);
                    a.invoke_apply_current();
                }
            }
        });
    }
}
