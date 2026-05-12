# ZeroIn

A lightweight, customizable crosshair overlay for Windows, written in Rust.

Renders a Direct2D-powered transparent overlay that sits on top of all your games and applications. Configure it once and toggle it from the system tray.

## Features

- **4 crosshair types** — Dot, Cross, T-shape, Circle
- **Fully configurable** — size, thickness, color (hex), opacity, center dot, border, gap width
- **Click-through overlay** — mouse events pass straight to the window behind
- **System tray** — toggle on/off, switch types, reload config without restarting
- **Persistent config** — reads `config.ini` next to the executable

## Usage

1. Download the latest release or build from source.
2. Place `config.ini` next to the executable (optional — defaults apply otherwise).
3. Run `ZeroIn.exe` — it lives in the system tray.
4. Right-click the tray icon to:
   - Toggle crosshair on/off
   - Switch crosshair type
   - Reload config

## Configuration

Edit `config.ini` (placed next to the executable):

```ini
[crosshair]
type = t              ; dot | cross | t | circle
size = 32             ; crosshair size in pixels
thickness = 1         ; line thickness
color = #22B8FF       ; hex color
dot_center = true     ; show center dot
opacity = 0.9         ; 0.0 to 1.0
border = false        ; outline mode (circle only)
space_width = 6       ; gap between center and crosshair arms
```

Default config applies if the file is missing or a value is invalid.

## Build from Source

**Requirements:**
- Rust edition 2024 (nightly)
- Windows (uses Win32 + Direct2D APIs)

```sh
git clone https://github.com/YOUR_USER/ZeroIn
cd ZeroIn
cargo build --release
```

The binary will be at `target/release/ZeroIn.exe`. Place `config.ini` and optionally `icon.ico` next to it.

## Technical

- Uses `windows` crate (Win32 API) for overlay window, Direct2D rendering, and tray icon
- Renders on a transparent layered window (`WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST`)
- Crosshair drawn with Direct2D primitives (ellipses, rectangles) via `UpdateLayeredWindow`

## License

MIT
