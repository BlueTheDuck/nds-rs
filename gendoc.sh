#!/bin/sh

python3 -m http.server 2>/dev/null &
inotifywait -r -m \
    -e close_write \
    src/ nds-sys/src nds-entry/src libc/src |
while read path action file; do
    cargo doc --offline --quiet --jobs 1 --workspace 2>/dev/null
done