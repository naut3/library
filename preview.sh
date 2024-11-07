#!/bin/sh
RUSTDOCFLAGS="--html-in-header katex.html" cargo doc --no-deps --all --release --open
