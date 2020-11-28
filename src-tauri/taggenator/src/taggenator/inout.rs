// TODO: remove
#![allow(warnings, unused)]

use std::io::{self, Write};

pub fn readline(message: &str) -> Option<String> {
	let mut ret = String::new();
	print!("{}", message);
	io::stdout().flush();
	io::stdin()
		.read_line(&mut ret)
		.expect("Failed to read from stdin");
	Some(ret)
}
