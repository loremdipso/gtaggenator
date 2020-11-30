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
#cargo build --verbose --release -p taggenator

# thanks! https://github.com/rust-lang/cargo/issues/3591#issuecomment-673356426
#cargo build --release 2>&1 | rg -i --multiline "(^error.*\n.*)|(aborting)|(warnings)"

cd ../tests
#time ~/.cargo-target/release/taggenator dump tags_inclusive yup yupp
#time ~/.cargo-target/release/taggenator dump tags_inclusive yupp yup
time ~/.cargo-target/release/taggenator dump touched
#time ~/.cargo-target/release/taggenator dump untouched
#time ~/.cargo-target/release/taggenator dump -sort alpha -sort least_frequently_opened
#time ~/.cargo-target/release/taggenator dump -sort alpha -sort biggest
#time ~/.cargo-target/release/taggenator dump -sort alpha -sort oldest
#time ~/.cargo-target/release/taggenator dump -sort alpha -sort newest -sort reverse
#time ~/.cargo-target/release/taggenator dump_tags search yup
#time ~/.cargo-target/release/taggenator dump search yup -sort reverse
#time ~/.cargo-target/release/taggenator dump search yup -sort random
#time ~/.cargo-target/release/taggenator dump search_inclusive yup sup
#time ~/.cargo-target/release/taggenator dump search 5718 -5718
#time ~/.cargo-target/release/taggenator dump search 5718 -sort search 5718
#time ~/.cargo-target/release/taggenator dump search 5718
#time ~/.cargo-target/release/taggenator dump_tags search 5718 -sort search 5718
#time ~/.cargo-target/release/taggenator dump_tags search 5718
#time ~/.cargo-target/release/taggenator dump_tags search
#time ~/.cargo-target/release/taggenator dump_tags

