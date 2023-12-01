#!/bin/sh
set -Eeo pipefail

TODAY="$(date +%d)"
FILENAME="src/day$TODAY.rs"
if [ ! -f "$FILENAME" ]; then
    sed "s/DAYNUM/$TODAY/g" template.rs > "$FILENAME"
fi
if ! grep -Fq "day$TODAY" Cargo.toml; then
    cat template.toml | sed "s/DAYNUM/$TODAY/g" >> Cargo.toml
fi
if [ ! -f "input/$TODAY" ]; then
    echo "Enter input:"
    cat > "input/$TODAY"
fi
cargo run --bin "day$TODAY"
