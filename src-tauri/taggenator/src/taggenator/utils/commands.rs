use crate::BError;
use std::io::prelude::*;
use std::io::{self, Write};
use std::process::Command;

pub fn run_command_string(command: &String) -> Result<String, BError> {
	let pieces = shell_words::split(&command)?;

	if pieces.len() > 0 {
		let mut builder = Command::new(pieces[0].to_string());
		for argument in pieces.iter().skip(1) {
			builder.arg(argument.clone());
		}

		// TODO: this is sus
		return Ok(String::from_utf8_lossy(&builder.output()?.stdout).to_string());
	}

	return Ok("".to_string());
}
