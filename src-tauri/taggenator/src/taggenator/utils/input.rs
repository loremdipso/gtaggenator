use std::io::prelude::*;
use std::io::{self, Write};

pub fn readline(message: &str) -> Result<String, std::io::Error> {
	print!("{}", message);
	io::stdout().flush();
	for line in io::stdin().lock().lines() {
		return Ok(line?.trim().to_string());
	}
	return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
}
