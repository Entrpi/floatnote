# FloatNote

A high-performance, frameless reference viewer for macOS (and Linux/Windows).
Built with Tauri v2 + Svelte + TypeScript + Rust.

## Features

- **Frameless UI**: The window is the image.
- **Always on Top**: Floats above other windows by default (Toggle via Context Menu).
- **Smart Resizing**: Automatically resizes the window to fit the dropped image.
- **Zooming**:
    - Scroll Mouse Wheel to Zoom In/Out.
    - Use `+` / `-` keys to Zoom.
- **Context Menu**: Right-click to access:
    - **Opacity**: Adjust transparency (25% - 100%).
    - **Always on Top**: Toggle pinning.
    - **Close**: Quit the app.
- **Cross-Platform Architecture**: Designed with a `WindowController` trait to handle platform-specific window constraints (e.g., aspect ratio locking on macOS).

## Prerequisites

### All Platforms
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js**: [Install Node.js](https://nodejs.org/)

### Linux
You need to install system dependencies for Tauri:
```bash
sudo apt-get install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## Development

1.  **Install dependencies**:
    ```bash
    npm install
    ```

2.  **Run in development mode**:
    ```bash
    npm run tauri dev
    ```

## Building

```bash
npm run tauri build
```

## Architecture Notes

- **Window Resizing**:
    - **macOS**: Uses `cocoa` and `objc` crates to strictly lock the window's aspect ratio at the OS level.
    - **Linux/Windows**: Falls back to "Internal Letterboxing". The window resizes to the image initially, but if forced to a different ratio, the image centers itself within the transparent window.

- **Frontend**:
    - Located in `src/`.
    - Uses Svelte for high-performance DOM updates without virtual DOM overhead.
