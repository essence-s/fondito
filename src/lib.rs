slint::include_modules!();
slint_layer_shell::windows![WallpaperPickerWindow];

mod app;
mod config;
mod ui;

use slint_layer_shell::{
    run_windows,
    layer_properties::{LayerAnchor, LayerType, WindowConf, BoardType},
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .init();

    let picker_conf = WindowConf::builder()
        .width(1366_u32)
        .height(768_u32)
        .anchor_1(
            LayerAnchor::TOP
                | LayerAnchor::BOTTOM
                | LayerAnchor::LEFT
                | LayerAnchor::RIGHT,
        )
        .exclusive_zone(-1)
        .layer_type(LayerType::Overlay)
        .board_interactivity(BoardType::OnDemand)
        .build()
        .unwrap();

    let picker = WallpaperPickerWindowWl::spawn("fondito", picker_conf);

    ui::adapters::connect_all(&picker);

    log::info!("=== fondito running ===");
    run_windows!(windows: [picker])?;
    Ok(())
}
