#!/bin/sh
set -Eeo pipefail

mkdir -p src/bin
mkdir -p input

DATE="${1:-$(date +%d)}"
FILENAME="src/bin/day$DATE.rs"
if [ ! -f "$FILENAME" ]; then
    sed "s/DAYNUM/$DATE/g" template.rs > "$FILENAME"
fi
if [ ! -f "input/$DATE.ex" ]; then
    echo "Enter example input:"
    cat > "input/$DATE.ex"
fi
if [ ! -f "input/$DATE" ]; then
    echo "Enter input:"
    cat > "input/$DATE"
fi
