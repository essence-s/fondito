use std::path::Path;
use std::process::Command;

pub fn generate_image_thumbnail(
    src: &Path,
    dst: &Path,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(src)?;
    let thumb = img.thumbnail(width, height);
    thumb.save(dst)?;
    Ok(())
}

pub fn extract_video_frame(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss",
            "00:00:01",
            "-i",
            &src.to_string_lossy(),
            "-vframes",
            "1",
            "-vf",
            "scale=600:420:force_original_aspect_ratio=decrease",
            &dst.to_string_lossy(),
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("ffmpeg failed: {}", stderr);
        return Err("ffmpeg failed to extract frame".into());
    }

    Ok(())
}
