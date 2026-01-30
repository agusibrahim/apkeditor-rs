# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rsapkeditor is a Rust + WebAssembly library for editing and signing Android APK files directly in the browser. All processing happens client-side with no server uploads.

## Build Commands

```bash
# Full build: compile WASM, copy to frontend, install deps, build UI
make build

# Development: compile WASM and start Vite dev server
make dev

# Clean all build artifacts
make clean

# Manual WASM build only
wasm-pack build --target web --features wasm --no-default-features

# Frontend only (after WASM is in frontend/src/wasm/)
cd frontend && bun install && bun run build

# Frontend dev server
cd frontend && bun run dev
```

## Architecture

### Rust Core (`/src`)
- `lib.rs` - WASM entry point with `#[wasm_bindgen]` exports
- `manifest.rs` - Binary AndroidManifest.xml (AXML) string pool editing
- `sign.rs` - APK Signature Scheme v2 implementation

### React Frontend (`/frontend`)
- `src/App.tsx` - Main UI with file upload, manifest editor, signing config
- `src/hooks/use-wasm.ts` - WASM module loader and TypeScript wrapper
- `src/components/ui/` - Shadcn UI components

### Build Output
- `pkg/` - Generated WASM bindings (copied to `frontend/src/wasm/` during build)
- `frontend/dist/` - Production build output

## Key WASM Exports

| Function | Purpose |
|----------|---------|
| `edit_apk` | Edit and sign with default debug key |
| `edit_apk_with_keystore` | Edit and sign with custom JKS/P12 keystore |
| `get_apk_info` | Extract package name, app name, version |
| `get_apk_icon` | Extract app icon as PNG bytes |
| `get_keystore_aliases` | List private key aliases from keystore |
| `verify_keystore_password` | Validate store password |
| `verify_key_password` | Validate key password for specific alias |

## Technical Notes

### Keystore Format Detection
JKS and PKCS12 formats are auto-detected by magic bytes:
- JKS: `0xfeedfeed` - requires password-based key decryption
- PKCS12: ASN.1 DER - raw key extraction works

### APK Alignment Requirements
- `resources.arsc`: 4-byte alignment
- `.so` files: 4096-byte page alignment

### Feature Flags
- `cli` (default) - Command-line binary
- `wasm` - WebAssembly build with JS bindings

## Frontend Stack
- React 18 + TypeScript
- Vite bundler
- Tailwind CSS + Shadcn UI
- Bun package manager (preferred over npm)
