use std::path::PathBuf;

pub fn get_wallpaper_dir() -> PathBuf {
    std::env::var("WALLPAPER_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join("Wallpapers")
        })
}

pub fn get_cache_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".cache/wallpaper_picker")
}

pub fn get_thumb_dir() -> PathBuf {
    let dir = get_cache_dir().join("thumbs");
    let _ = std::fs::create_dir_all(&dir);
    dir
}
