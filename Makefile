.PHONY: all build clean

# Default target
all: clean build

# Build target
build:
	@echo "🦀 Building WASM module..."
	wasm-pack build --target web --features wasm --no-default-features
	@echo "📂 Preparing dist folder..."
	mkdir -p dist
	cp web/index.html dist/
	cp web/manifest.json dist/
	cp web/sw.js dist/
	cp web/icon.svg dist/
	cp -r web/styles.css dist/ 2>/dev/null || :
	cp -r pkg dist/
	@echo "✅ Build complete! Output in 'dist/'"

# Clean target
clean:
	@echo "🧹 Cleaning..."
	rm -rf dist
	rm -rf pkg
	rm -rf web/pkg
