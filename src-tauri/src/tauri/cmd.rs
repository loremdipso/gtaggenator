use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use taggenator::taggenator::database::CACHE_FILENAME;
use taggenator::taggenator::database::DATABASE_FILENAME;
use taggenator::taggenator::models::record::Record;
use taggenator::BError;

#[derive(Deserialize, Debug)]
pub struct DoSomethingPayload {
	command: String,
	state: String,
	data: u64,
}

// The commands definitions
// Deserialized from JS
// #[serde(tag = "cmd", rename_all = "camelCase")]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
	// as an example
	DoSomethingSync {
		argument: String,
	},

	GetStartupOptions {
		callback: String,
		error: String,
	},

	OpenNewFolder {
		callback: String,
		error: String,
	},

	RemoveFolder {
		callback: String,
		error: String,
		path: String,
	},

	Initialize {
		callback: String,
		error: String,
		location: String,
	},

	AddTags {
		callback: String,
		error: String,
		record: Record,
		tag_line: String,
	},

	GetTags {
		callback: String,
		error: String,
	},

	GetRecords {
		callback: String,
		error: String,
		args: Vec<String>,
	},

	OpenRecord {
		callback: String,
		error: String,
		record: Record,
	},

	GetGrabBag {
		callback: String,
		error: String,
		record: Record,
	},

	SetGrabBagKey {
		callback: String,
		error: String,
		record: Record,
		key: String,
		value: String,
	},

	GetInitialArguments {
		callback: String,
		error: String,
	},

	GetPort {
		callback: String,
		error: String,
	},

	OpenContainingFolder {
		callback: String,
		error: String,
		location: String,
	},

	OpenNatively {
		callback: String,
		error: String,
		location: String,
	},

	EditSettings {
		callback: String,
		error: String,
	},

	Reload {
		callback: String,
		error: String,
	},

	GetRecommendedTags {
		callback: String,
		error: String,
		record: Record,
	},

	GetCache {
		callback: String,
		error: String,
		key: String,
	},

	SetCache {
		callback: String,
		error: String,
		key: String,
		value: String,
	},
}

#[derive(Serialize)]
pub struct Response<'a> {
	pub value: u64,
	pub message: &'a str,
}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
pub struct CommandError<'a> {
	message: &'a str,
}

impl<'a> CommandError<'a> {
	pub fn new(message: &'a str) -> Self {
		Self { message }
	}
}

impl<'a> std::fmt::Display for CommandError<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

#[derive(Serialize)]
pub struct StartupOptions {
	pub folders: Vec<StartupFolder>,
	pub skip: bool,
}

#[derive(Serialize)]
pub struct StartupFolder {
	pub location: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
	pub opened: Vec<String>,
}

// TODO: refactor
pub fn get_locations() -> Result<Vec<StartupFolder>, BError> {
	let mut folders = vec![];

	let base = std::env::current_dir()?;
	if Path::join(&base, DATABASE_FILENAME).exists() {
		folders.push(StartupFolder {
			location: base.to_string_lossy().to_string(),
		});
	}

	let home_dir = std::env::home_dir().unwrap();
	let config_path = Path::new(&home_dir).join(CACHE_FILENAME);
	if config_path.exists() {
		let mut file = File::open(config_path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)
			.unwrap_or_else(|err| panic!("Error while reading cache: [{}]", err));

		let mut cache: Cache = serde_yaml::from_str(&contents)?;
		for path in &mut cache.opened {
			if !folders.iter().any(|el| el.location.eq(path)) {
				folders.push(StartupFolder {
					location: path.to_string(),
				});
			}
		}
	}

	return Ok(folders);
}

pub fn add_to_cache(location: String) -> Result<(), BError> {
	let home_dir = std::env::home_dir().unwrap();
	let config_path = Path::new(&home_dir).join(CACHE_FILENAME);
	if !config_path.exists() {}

	let cache = if config_path.exists() {
		let mut file = File::open(&config_path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)
			.unwrap_or_else(|err| panic!("Error while reading cache: [{}]", err));

		let mut cache: Cache = serde_yaml::from_str(&contents)?;
		if !cache.opened.iter().any(|el| el == &location) {
			cache.opened.push(location.to_string());
		}
		cache
	} else {
		Cache {
			opened: vec![location],
		}
	};

	let s = serde_yaml::to_string(&cache)?;
	std::fs::write(config_path, s)?;

	return Ok(());
}

pub fn remove_from_cache(location: String) -> Result<bool, BError> {
	let home_dir = std::env::home_dir().unwrap();
	let config_path = Path::new(&home_dir).join(CACHE_FILENAME);
	if !config_path.exists() {}

	if config_path.exists() {
		let mut file = File::open(&config_path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)
			.unwrap_or_else(|err| panic!("Error while reading cache: [{}]", err));

		let mut cache: Cache = serde_yaml::from_str(&contents)?;
		let index = cache.opened.iter().position(|e| **e == *location);
		if let Some(index) = index {
			cache.opened.remove(index);

			let s = serde_yaml::to_string(&cache)?;
			std::fs::write(config_path, s)?;
			return Ok(true);
		}
	}

	return Ok(false);
}
