use slint::{ComponentHandle, Image, Model, ModelRc, VecModel};
use crate::config;
use crate::app::wallpaper_picker::services;

pub struct PickerController;

impl PickerController {
    pub fn connect(window: &crate::WallpaperPickerWindowSpell) {
        let adapter = window.global::<crate::PickerAdapter>();
        let weak = window.as_weak();

        let raw_items = services::scanner::scan_wallpapers();
        services::scanner::ensure_thumbnails(&raw_items);
        let colors = services::color::extract_dominant_colors(&raw_items);

        let wp_items: Vec<crate::WallpaperItem> = raw_items
            .into_iter()
            .enumerate()
            .filter_map(|(i, (name, url, vid))| {
                let path_str = url.strip_prefix("file://").unwrap_or(&url);
                let path = std::path::Path::new(&path_str);
                let image = Image::load_from_path(path).ok()?;
                Some(crate::WallpaperItem {
                    file_name: name.into(),
                    file_url: image,
                    is_video: vid,
                    dominant_color: colors.get(i).cloned().unwrap_or("#A9A9A9".to_string()).into(),
                })
            })
            .collect();

        let shared_dir = config::get_wallpaper_dir();
        let model = ModelRc::new(VecModel::from(wp_items));
        let transition_list: Vec<&str> = vec![
            "simple", "fade", "left", "right", "top", "bottom", "wipe",
            "grow", "center", "outer", "random", "wave",
        ];

        adapter.set_wallpaper_model(model.clone());
        adapter.set_wallpaper_count(model.row_count() as i32);
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
            let model = model.clone();
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
            let wd = shared_dir.clone();
            let transitions = transition_list.clone();
            let weak = weak.clone();
            move || {
                if let Some(w) = weak.upgrade() {
                    let a = w.global::<crate::PickerAdapter>();
                    let idx = a.get_current_index();
                    let model = a.get_wallpaper_model();
                    if let Some(item) = model.row_data(idx as usize) {
                        let monitors = services::apply::get_monitors();
                        services::apply::apply_wallpaper(
                            &item.file_name,
                            &wd,
                            item.is_video,
                            &monitors,
                            &transitions,
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
