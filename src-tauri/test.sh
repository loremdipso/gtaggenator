#!/bin/bash

#cd ./test
#rm -rf ./output
#mkdir ./output &>/dev/null
#mkdir ./output/png &>/dev/null
#mkdir ./output/jpg &>/dev/null

set -e

#image_name = "BROKEN.jpg"
#image_name="TB.jpg"
cd ./taggenator
cargo build --release -q -p taggenator

# thanks! https://github.com/rust-lang/cargo/issues/3591#issuecomment-673356426
#cargo build --release 2>&1 | rg -i --multiline "(^error.*\n.*)|(aborting)|(warnings)"

cd ../tests
time cargo run -p taggenator --release -q -- dump_tags

