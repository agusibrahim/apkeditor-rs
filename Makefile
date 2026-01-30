.PHONY: all build clean dev

# Default target
all: clean build

# Build target
build:
	@echo "🦀 Building WASM module..."
	wasm-pack build --target web --features wasm --no-default-features
	@echo "📦 Copying WASM to frontend..."
	cp -r pkg frontend/src/wasm
	@echo "📦 Installing frontend dependencies..."
	cd frontend && bun install
	@echo "🏗️ Building frontend..."
	cd frontend && bun run build
	@echo "✅ Build complete! Output in 'frontend/dist/'"

# Development mode
dev:
	@echo "🦀 Building WASM module..."
	wasm-pack build --target web --features wasm --no-default-features
	@echo "📦 Copying WASM to frontend..."
	cp -r pkg frontend/src/wasm
	@echo "🚀 Starting dev server..."
	cd frontend && bun run dev

# Clean target
clean:
	@echo "🧹 Cleaning..."
	rm -rf frontend/dist
	rm -rf frontend/src/wasm
	rm -rf pkg
