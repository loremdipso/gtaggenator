// TODO: remove
#![allow(warnings, unused)]

use crate::taggenator::errors::BError;
use crate::taggenator::inout::readline;
use jwalk;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{error::Error, include_str};
use toml::Value;
use walkdir;

static SETTINGS_FILENAME: &str = "tsettings.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
	pub extensions: Vec<String>,
	pub synonyms: HashMap<String, String>,
	pub prefixes: Vec<String>,
	pub derived: HashMap<String, Vec<String>>,
	pub commands: HashMap<String, String>,
	pub tagger: HashMap<String, String>,
	pub openerconfig: HashMap<String, HashMap<String, String>>,
}

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

		Settings::load(SETTINGS_FILENAME)
	}

	fn get_default() -> &'static str {
		include_str!("../data/tsettings.yaml")
	}

	pub fn load(path: &str) -> Result<Settings, BError> {
		match File::open(&path) {
			Ok(mut file) => {
				let mut content = String::new();
				file.read_to_string(&mut content)
					.unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

				let settings = serde_yaml::from_str(&content)?;
				return Ok(settings);
			}
			Err(error) => {
				return Err(Box::new(error));
				// error!("Could not find config file, using default!");
				// return Config::new();
			}
		};
	}

	pub fn save(&self) -> Result<(), BError> {
		let s = serde_yaml::to_string(&self)?;
		fs::write(SETTINGS_FILENAME, s)?;
		Ok(())
	}
}
