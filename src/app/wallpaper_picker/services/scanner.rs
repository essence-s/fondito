use std::path::Path;
use crate::config;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif", "bmp"];
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "mov", "webm", "avi"];

pub fn is_image_ext(ext: &str) -> bool {
    IMAGE_EXTENSIONS.contains(&ext)
}

pub fn is_video_ext(ext: &str) -> bool {
    VIDEO_EXTENSIONS.contains(&ext)
}

pub fn scan_wallpapers() -> Vec<(String, String, bool)> {
    let wallpaper_dir = config::get_wallpaper_dir();
    let mut items = Vec::new();

    if !wallpaper_dir.exists() {
        log::warn!("Wallpaper directory does not exist: {:?}", wallpaper_dir);
        return items;
    }

    let entries = match std::fs::read_dir(&wallpaper_dir) {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to read wallpaper directory: {}", e);
            return items;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let is_video = is_video_ext(&ext);
        if !is_video && !is_image_ext(&ext) {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.to_string())
            .unwrap_or_default();

        let thumb_name = if is_video {
            let stem = Path::new(&file_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(&file_name);
            format!("000_{}.png", stem)
        } else {
            file_name.clone()
        };
        let thumb_url = format!("file://{}", config::get_thumb_dir().join(&thumb_name).display());

        items.push((file_name, thumb_url, is_video));
    }

    items.sort_by(|a, b| a.0.cmp(&b.0));
    items
}

pub fn ensure_thumbnails(items: &[(String, String, bool)]) {
    let src_dir = config::get_wallpaper_dir();

    for (file_name, thumb_url, is_video) in items {
        let thumb_path_str = thumb_url.strip_prefix("file://").unwrap_or(thumb_url);
        let thumb_path = Path::new(&thumb_path_str);

        if thumb_path.exists() {
            continue;
        }

        let src_path = src_dir.join(file_name);

        let result = if *is_video {
            super::thumbnails::extract_video_frame(&src_path, thumb_path)
        } else {
            super::thumbnails::generate_image_thumbnail(&src_path, thumb_path, 600, 420)
        };

        if let Err(e) = result {
            log::warn!("Failed to generate thumbnail for {}: {}", file_name, e);
        }
    }
}
