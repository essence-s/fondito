# fondito — Wallpaper Picker (Slint + Rust)

## Crates externos clave

### `slint = "1.16.1"` (+ `slint-build`)

GUI toolkit con renderer Skia. Compilación en `build.rs`:

```rust
slint_build::compile("src/ui/app.slint").unwrap();
```

### `spell-framework = "1.0.5"`

Maneja la conexión Wayland (layer-shell). Genera wrappers spell para cada ventana:

```rust
spell_framework::generate_widgets![WallpaperPickerWindow];
```

### `image = "0.25"`, `serde_json = "1"`, `fastrand = "2"`

Thumbnails, parseo de `hyprctl monitors -j`, transiciones aleatorias.

## Build & Run

```sh
cargo run
```

## Widget Architecture Pattern

Cada widget sigue este patrón (`src/app/<widget>/`):

### 1. UI (`<widget>.slint`) — Solo declarativa

Define un **`global`** propio para el estado y callbacks, y el componente visual que lo consume directamente:

```slint
export global WallpaperPickerAdapter {
    in-out property <[WallpaperItem]> wallpaper-model;
    in-out property <int> current-index;
    in-out property <string> current-filter: "All";
    callback navigate-left;
    callback apply-current;
    // ...
}

export component WallpaperPickerWindow inherits Window {
    // Usa WallpaperPickerAdapter directamente
    FocusScope {
        key_pressed(event) => {
            if (event.text == Key.RightArrow) {
                WallpaperPickerAdapter.navigate-right();
            }
        }
    }
}
```

Reglas:

- El `global` **se exporta** (para que Rust pueda accederlo)
- El componente **no tiene** `callback` propio — usa el global directo
- El componente **no recibe** `in property` de state — lee del global

### 2. Controller (`mod.rs`) — Lógica Rust

```rust
use slint::ComponentHandle;
use crate::wallpaper;

pub struct WallpaperPickerController;

impl WallpaperPickerController {
    pub fn connect(window: &crate::WallpaperPickerWindowSpell) {
        let adapter = window.global::<crate::WallpaperPickerAdapter>();
        adapter.on_navigate_left(move || { /* ... */ });
    }
}
```

### 3. Adapter central (`src/ui/adapters.rs`)

Orquesta todos los controllers:

```rust
use crate::app::wallpaper_picker::WallpaperPickerController;

pub fn connect_all(window: &crate::WallpaperPickerWindowSpell) {
    WallpaperPickerController::connect(window);
}
```

### 4. Entry point (`src/ui/app.slint`)

Importa y re-exporta los globals para que Slint genere el Rust code:

```slint
import { WallpaperPickerAdapter, WallpaperPickerWindow, WallpaperItem } from "../app/wallpaper_picker/picker.slint";
export { WallpaperPickerAdapter, WallpaperPickerWindow, WallpaperItem }
```

### 5. Main (`src/main.rs`) — solo init

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fondito::run()
}
```

### 6. Lib (`src/lib.rs`) — setup

```rust
slint::include_modules!();
spell_framework::generate_widgets![WallpaperPickerWindow];

mod app;
mod config;
mod ui;
mod wallpaper;

pub fn run() {
    let cfg = WindowConf::builder()/* ... */;
    let picker = WallpaperPickerWindowSpell::invoke_spell("spell-wallpaper", cfg);
    ui::adapters::connect_all(&picker);
    cast_spell!(windows: [picker])
}
```

## Flujo completo

```
Usuario interactúa
  → picker.slint: TouchArea / FocusScope.key_pressed
    → WallpaperPickerAdapter.navigate-right()
      → Controller::connect() (Rust)
        → wallpaper::apply / scanner / color
          → adapter.set_current_index(...) → UI se actualiza sola
```

## Estructura de carpetas

```
src/
├── main.rs                 # Entry point (thin)
├── lib.rs                  # include_modules!() + run()
├── config.rs               # get_wallpaper_dir, get_cache_dir, get_thumb_dir
├── wallpaper/              # Business logic layer
│   ├── mod.rs
│   ├── scanner.rs          # scan_wallpapers, ensure_thumbnails
│   ├── thumbnails.rs       # generate_image_thumbnail, extract_video_frame
│   ├── color.rs            # extract_dominant_colors, get_hsl_bucket
│   └── apply.rs            # apply_wallpaper, get_monitors
├── ui/
│   ├── app.slint           # Entry: imports + exports globals
│   ├── adapters.rs         # connect_all() — orquesta controllers
│   ├── icons.slint         # Icon components (Spinner, IconSearch, etc.)
│   └── mod.rs
├── app/
│   ├── mod.rs
│   └── wallpaper_picker/
│       ├── picker.slint    # Main window + WallpaperPickerAdapter global
│       ├── card.slint      # WallpaperCard
│       ├── filter_bar.slint # FilterBar + NotificationDrawer, SearchBox
│       └── mod.rs          # WallpaperPickerController
└── build.rs                # slint_build::compile("src/ui/app.slint")
```

## Comandos útiles

```sh
cargo build
cargo run
```
