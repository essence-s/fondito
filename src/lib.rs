slint::include_modules!();
spell_framework::generate_widgets![WallpaperPickerWindow];

mod app;
mod config;
mod ui;
mod wallpaper;

use spell_framework::{
    cast_spell,
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

    let picker = WallpaperPickerWindowSpell::invoke_spell("fondito", picker_conf);

    ui::adapters::connect_all(&picker);

    log::info!("=== fondito running ===");
    cast_spell!(windows: [picker])?;
    Ok(())
}
