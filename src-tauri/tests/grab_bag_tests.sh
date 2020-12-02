#!/bin/bash

for i in $(seq 1 10000)
do
	echo $i
	~/.cargo-target/release/taggenator grabbag add "./tmp/B.txt" key_$i value_$i --ignore-update
done

