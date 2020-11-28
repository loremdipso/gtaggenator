// TODO: remove
#![allow(dead_code, warnings, unused)]
use std::env;
use std::error::Error;
use taggenator::Taggenator;

fn main() -> Result<(), Box<Error>> {
	let args: Vec<String> = env::args().skip(1).collect();
	let mut taggenator = Taggenator::new().unwrap();
	taggenator.parse_args(args)
}
