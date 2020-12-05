#!/bin/bash

set -e

cd ./taggenator

# debug build
cargo build -q -p taggenator

cd ../tests

valgrind --tool=massif ~/.cargo-target/debug/gtaggenator open
#valgrind --tool=massif ~/.cargo-target/debug/taggenator open
