#!/bin/sh
rm -rf test/
oj d $1
oj t -c "cargo run --release --bin contest"
