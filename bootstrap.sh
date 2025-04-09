#!/usr/bin/env bash

if command -v cargo-make > /dev/null 2>&1
then
    echo "✅ cargo-make is already installed."
else
    echo "🧰 Installing cargo-make..."
    cargo install cargo-make
fi

cargo make dev
