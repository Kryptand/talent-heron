#!/bin/bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 "$(dirname "$0")/src-tauri/target/release/talent-heron" "$@"
