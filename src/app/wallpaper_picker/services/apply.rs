use std::path::Path;
use std::process::Command;

pub fn get_monitors() -> Vec<String> {
    let output = match Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            log::warn!("Failed to run hyprctl: {}", e);
            return vec!["eDP-1".to_string()];
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    match serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
        Ok(monitors) => monitors
            .iter()
            .filter_map(|m| m.get("name")?.as_str().map(|s| s.to_string()))
            .collect(),
        Err(e) => {
            log::warn!("Failed to parse hyprctl output: {}", e);
            vec!["eDP-1".to_string()]
        }
    }
}

pub fn apply_wallpaper(
    file_name: &str,
    src_dir: &Path,
    is_video: bool,
    monitors: &[String],
    transitions: &[&str],
) {
    // let clean_name = file_name.strip_prefix("000_").unwrap_or(file_name);
    // let file_path = src_dir.join(clean_name);
    let file_path = src_dir.join(file_name);

    if !file_path.exists() {
        log::error!("Wallpaper file not found: {:?}", file_path);
        return;
    }

    let transition = transitions[fastrand::usize(..transitions.len())];
    let monitor_args = if monitors.is_empty() || monitors.len() == 1 {
        vec!["img".to_string()]
    } else {
        let output = monitors.join(",");
        vec!["img".to_string(), "-o".to_string(), output]
    };

    if is_video {
        let _ = Command::new("pkill")
            .arg("mpvpaper")
            .output();

        for mon in monitors {
            let mut cmd = Command::new("mpvpaper");
            cmd.args([
                "-o",
                "no-audio --loop --input-ipc-server=/tmp/mpvsocket --hwdec=auto --profile=high-quality",
                mon,
                &file_path.to_string_lossy(),
            ]);
            if let Err(e) = cmd.spawn() {
                log::error!("Failed to start mpvpaper: {}", e);
            }
        }
    } else {
        let _ = Command::new("pkill")
            .arg("mpvpaper")
            .output();

        let mut cmd = Command::new("swww");
        cmd.args(&monitor_args);
        cmd.args([
            &file_path.to_string_lossy(),
            "--transition-type",
            transition,
            "--transition-pos",
            "0.5,0.5",
            "--transition-fps",
            "144",
            "--transition-duration",
            "1",
        ]);

        if let Err(e) = cmd.spawn() {
            log::error!("Failed to run swww: {}", e);
        }
    }
}
