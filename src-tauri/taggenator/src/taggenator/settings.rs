// TODO: remove
#![allow(warnings, unused)]

use crate::taggenator::errors::BError;
use crate::taggenator::inout::readline;
use jwalk;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, include_str};
use toml::Value;
use walkdir;

static SETTINGS_FILENAME: &str = "taggenator_settings.toml";

pub struct Settings {}

impl Settings {
	pub fn new() -> Result<Settings, Box<Error>> {
		if !Path::new(SETTINGS_FILENAME).exists() {
			let ret = readline(
				format!(
					"{} doesn't exist.\nShall I create a new one? (y/n)> ",
					SETTINGS_FILENAME
				)
				.as_str(),
			)
			.unwrap();

			if ret.trim().to_lowercase() == "y" {
				println!("Creating...");
				fs::write(SETTINGS_FILENAME, Settings::get_default())
					.expect("Unable to write file");
			}
		}
		return Ok(Settings {});
	}

	fn get_default() -> &'static str {
		include_str!("../data/taggenator_settings.toml")
	}

	/// Attempt to load and parse the config file into our Config struct.
	/// If a file cannot be found, return a default Config.
	/// If we find a file but cannot parse it, panic
	pub fn parse(self, path: String) -> Result<(), BError> {
		match File::open(&path) {
			Ok(mut file) => {
				let mut toml_content = String::new();
				file.read_to_string(&mut toml_content)
					.unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

				let package_info: Value = toml::from_str(&toml_content)?;
				// let toml = parser.parse();

				// if toml.is_none() {
				// 	for err in &parser.errors {
				// 		let (loline, locol) = parser.to_linecol(err.lo);
				// 		let (hiline, hicol) = parser.to_linecol(err.hi);
				// 		println!(
				// 			"{}:{}:{}-{}:{} error: {}",
				// 			path, loline, locol, hiline, hicol, err.desc
				// 		);
				// 	}
				// 	panic!("Exiting server");
				// }

				// let config = Value::Table(toml.unwrap());
				// match toml::decode(config) {
				// 	Some(t) => t,
				// 	None => panic!("Error while deserializing config"),
				// }
			}
			Err(error) => {
				return Err(Box::new(error));
				// error!("Could not find config file, using default!");
				// return Config::new();
			}
		};

		Ok(())
	}
}
