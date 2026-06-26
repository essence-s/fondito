use std::path::Path;

pub fn extract_dominant_colors(items: &[(String, String, bool)]) -> Vec<String> {
    items
        .iter()
        .map(|(_file_name, thumb_url, _is_video)| {
            let thumb_path_str = thumb_url.strip_prefix("file://").unwrap_or(thumb_url);
            let thumb_path = Path::new(thumb_path_str);

            if !thumb_path.exists() {
                return "#A9A9A9".to_string();
            }

            match extract_color_from_image(thumb_path) {
                Some(hex) => hex,
                None => "#A9A9A9".to_string(),
            }
        })
        .collect()
}

fn extract_color_from_image(path: &Path) -> Option<String> {
    let img = image::open(path).ok()?;
    let small = img.resize_exact(1, 1, image::imageops::FilterType::Nearest);
    let rgba = small.to_rgba8();
    let pixel = rgba.get_pixel(0, 0);
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    Some(hex)
}
