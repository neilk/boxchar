#!/bin/bash
set -e

echo "🌐 Building WASM for web..."

# Build the WASM package
echo "📦 Building WASM package..."
wasm-pack build --target web --out-dir pkg

# Create web directory if it doesn't exist
echo "📁 Setting up web directory..."
mkdir -p web

# Copy files to web directory
echo "📋 Copying files..."
cp -r pkg web/
cp data/dictionary.txt web/

echo "✅ Web build complete!"
echo "🚀 Files are ready in the 'web/' directory"
echo "💡 To serve: cd web && npx http-server -p 8000"