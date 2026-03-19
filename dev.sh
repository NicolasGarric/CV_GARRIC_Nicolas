#!/bin/bash
# Lance le SCSS en arrière-plan et le serveur Rust au premier plan

echo "🎨  Démarrage Sass..."
sass --watch static/scss/main.scss static/css/main.css &
SASS_PID=$!

echo "🚀  Démarrage serveur Rust..."
cargo run

# Quand cargo run s'arrête (Ctrl+C), on arrête Sass aussi
kill $SASS_PID
