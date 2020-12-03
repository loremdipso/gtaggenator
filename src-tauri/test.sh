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

#cargo test --release -p taggenator

#echo thing | ~/.cargo-target/release/taggenator open
#~/.cargo-target/release/taggenator open
#time ~/.cargo-target/release/taggenator grabbag add "./tmp/B.txt" huh welp2 --ignore-update
#time ~/.cargo-target/release/taggenator grabbag delete "./tmp/B.txt" sup
#time ~/.cargo-target/release/taggenator grabbag get "./tmp/B.txt" sup
#time ~/.cargo-target/release/taggenator grabbag get_all "./tmp/B.txt"
#time ~/.cargo-target/release/taggenator grabbag get_all "./tmp/B.txt"
#time ~/.cargo-target/release/taggenator grabbag get_all "/home/madams/Projects/gtaggenator/src-tauri/tests/tmp/B.txt"
#~/.cargo-target/release/taggenator dump -sort limit 20
#~/.cargo-target/release/taggenator dump -sort limit -20
echo add_record location 42 42 42 false "2020-11-11 11:18:18.123456 -0700 MST" DATE DATE | ~/.cargo-target/release/taggenator import
#echo add_record location 42 42 42 false "2020-11-11 11:18:18:18.470445054 -0700 MST" DATE DATE | ~/.cargo-target/release/taggenator import

#echo a add_tag location tag add_tag location2 tag2 | ~/.cargo-target/release/taggenator import
#echo test | ~/.cargo-target/release/taggenator open
#echo pre | ~/.cargo-target/release/taggenator open
#echo tag1,tag1,tag2 | ~/.cargo-target/release/taggenator open
#time ~/.cargo-target/release/taggenator dump tags_inclusive yup yupp
#time ~/.cargo-target/release/taggenator dump tags_inclusive yupp yup
#echo tag1, tag2 | time ~/.cargo-target/release/taggenator open
#echo -tag1 | ~/.cargo-target/release/taggenator open
#time ~/.cargo-target/release/taggenator dump touched
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

