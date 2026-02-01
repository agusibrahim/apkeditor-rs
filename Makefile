.PHONY: all build clean dev wasm copy-wasm serve serve-dist watch

# Default target
all: clean build

# Build WASM only
wasm:
	@echo "🦀 Building WASM module..."
	wasm-pack build --target web --features wasm --no-default-features
	@echo "✅ WASM built in 'pkg/'"

# Copy WASM to frontend
copy-wasm:
	@echo "📦 Copying WASM to frontend..."
	rm -rf frontend/src/wasm/*
	cp -r pkg/* frontend/src/wasm/
	cp pkg/rsapkeditor.js frontend/src/wasm/
	@echo "✅ WASM copied to 'frontend/src/wasm'"

# Build target (full production build)
build: wasm copy-wasm
	@echo "📦 Installing frontend dependencies..."
	cd frontend && bun install
	@echo "🏗️ Building frontend..."
	cd frontend && bun run build
	@echo "✅ Build complete! Output in 'frontend/dist/'"

# Development mode - build WASM + copy + serve dev server
dev: wasm copy-wasm
	@echo "🚀 Starting dev server..."
	cd frontend && bun run dev

# Quick dev server (without rebuilding WASM)
serve:
	@echo "🚀 Starting dev server..."
	cd frontend && bun run dev

# Serve production build locally
serve-dist: build
	@echo "🌐 Serving production build at http://localhost:8080"
	cd frontend/dist && python3 -m http.server 8080

# Watch mode (rebuild WASM on changes)
watch:
	@echo "👀 Watching for changes..."
	@which cargo-watch > /dev/null || (echo "❌ cargo-watch not installed. Install with: cargo install cargo-watch" && exit 1)
	cargo watch -x "wasm-pack build --target web --features wasm --no-default-features" -s "make copy-wasm"

# Clean target
clean:
	@echo "🧹 Cleaning..."
	rm -rf frontend/dist
	rm -rf frontend/src/wasm
	rm -rf pkg
	@echo "✅ Clean complete"
