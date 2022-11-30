#!/bin/sh
# Generate documentation for the projects
# limited to these 3 crates.
# Automatically watches the source files for changes
# and regenerates the documentation.

cargo_doc() {
    cargo doc --offline --quiet --jobs 1 --workspace --no-deps -p nds-rs -p nds-sys -p nds-proc-macros || exit 1
}

cargo_doc
cargo doc --workspace --open

# python3 -m http.server 2>/dev/null &
inotifywait -r -m \
    -e close_write \
    src/ nds-sys/src nds-proc-macros/src libc/src |
while read path action file; do
    cargo_doc 2>/dev/null
done