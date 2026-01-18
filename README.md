# rsapkeditor

A pure **Rust** + **WebAssembly** library and web interface for editing Android APK files directly in the browser.

🔗 **Live Demo**: [apkeditor-rs](https://agusibrahim.github.io/apkeditor-rs/)

## Features

- **100% Offline**: All processing happens in your browser. No APKs are uploaded to any server.
- **Manifest Editing**:
  - **Package Name**: Globally replaces the package name in the AndroidManifest string pool (fixes `INSTALL_FAILED_DUPLICATE_PERMISSION`).
  - **App Name**: Renames the application label.
  - **Version Info**: Updates `versionCode` and `versionName`.
- **Icon Preview**:
  - Displays the app icon even from obfuscated APKs.
  - Smart detection (Launcher -> Mipmap -> File Size heuristics).
- **Auto-Signing**: Automatically signs the edited APK with a built-in debug keystore (scheme v2).

## Build

This project uses `make` for build automation.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- `wasm-pack`: `cargo install wasm-pack`

### Commands

```bash
# Build WASM and prepare dist folder
make

# Clean build artifacts
make clean
```

The output will be in the `dist/` folder, ready for deployment.

## Technical Details

- **Core**: Built with Rust using `apk`, `zip`, `rsa`, `sha2` crates.
- **WASM**: Exposed via `wasm-bindgen`.
- **Manifest**: Uses direct binary editing of the `AndroidManifest.xml` via `apk` crate structures.
- **Frontend**: Vanilla HTML/JS/CSS (no bloated frameworks).

## License

MIT
