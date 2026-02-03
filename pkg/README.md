# APK Editor

A pure **Rust** + **WebAssembly** APK editor with a modern React frontend for editing Android APK files directly in the browser.

🔗 **Live Demo**: [apkeditor-rs](https://agusibrahim.github.io/apkeditor-rs/)

## Features

- **100% Offline**: All processing happens in your browser. No APKs are uploaded to any server.
- **PWA Support**: Install as a Progressive Web App for offline access.
- **Non-blocking Processing**: Web Worker ensures smooth UI even for large APKs (80MB+).
- **Manifest Editing**:
  - **Package Name**: Smart replacement that preserves class names while updating package references and permissions.
  - **App Name**: Renames the application label.
  - **Version Info**: Updates `versionCode` and `versionName`.
- **Advanced Signing Options**:
  - **Debug Key**: Default signing with built-in debug keystore (APK Signature Scheme v2).
  - **Custom Keystore**: Support for `.keystore`, `.jks`, `.p12`, `.pfx` files with real-time password validation.
- **Icon Preview**:
  - Displays the app icon even from obfuscated APKs.
  - Smart detection prioritizing higher density variants.

## Tech Stack

- **Backend**: Rust compiled to WebAssembly
  - `apk`, `zip`, `rsa`, `sha2`, `jks` crates
  - Binary editing of `AndroidManifest.xml`
- **Frontend**: React + TypeScript + Vite
  - Tailwind CSS + shadcn/ui components
  - Responsive design (mobile & desktop)

## Build

This project uses `make` for build automation.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/) (or npm/pnpm)
- `wasm-pack`: `cargo install wasm-pack`

### Commands

```bash
# Build WASM and frontend (production)
make

# Development mode with hot reload
make dev

# Clean build artifacts
make clean
```

The output will be in `frontend/dist/`, ready for deployment.

## Project Structure

```
├── src/                # Rust source code
│   ├── lib.rs          # WASM bindings
│   ├── manifest.rs     # AndroidManifest.xml editing
│   └── sign.rs         # APK signing logic
├── frontend/           # React frontend
│   ├── src/
│   │   ├── App.tsx     # Main application
│   │   ├── hooks/      # Custom React hooks
│   │   └── workers/    # Web Workers for WASM processing
│   └── public/         # Static assets
└── Makefile            # Build automation
```

## License

MIT
