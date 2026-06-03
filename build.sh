#!/usr/bin/env sh
set -eu

unset CDPATH
SCRIPT_DIR=$(cd -- "$(dirname -- "$0")" && pwd)
cd "$SCRIPT_DIR"

printf '%s\n' '[BUILD] Compiling mender...'
rustup run nightly cargo build -Z json-target-spec --release --target-dir "$SCRIPT_DIR/target" --manifest-path "$SCRIPT_DIR/mender/Cargo.toml" --bin mender

if [ ! -f "$SCRIPT_DIR/target/release/mender" ]; then
    printf '%s\n' '[ERROR] Expected target/release/mender was not created.'
    exit 1
fi

cp -f "$SCRIPT_DIR/target/release/mender" "$SCRIPT_DIR/mender-tool"
chmod +x "$SCRIPT_DIR/mender-tool"
printf '[BUILD] Output created at "%s"\n' "$SCRIPT_DIR/mender-tool"



