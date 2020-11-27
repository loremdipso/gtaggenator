#!/bin/bash

cd ./test
rm -rf ./output
mkdir ./output &>/dev/null
mkdir ./output/png &>/dev/null
mkdir ./output/jpg &>/dev/null

set -e

#image_name = "BROKEN.jpg"
#image_name="TB.jpg"
image_name="DIAG.png"
cargo build --release
#time cargo run --release -- --black-lines ./input/$image_name --output ./output/jpg
#time cargo run --release -q ./input/BROKEN.jpg --output ./output/jpg

time cargo run --release -q -- --black-lines ./input/*.jpg --output ./output/jpg

