#![allow(dead_code)]
use std::env;
use taggenator::Taggenator;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let taggenator = Taggenator::new();
	taggenator.parse_args(args);
}
