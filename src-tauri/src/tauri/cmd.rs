use serde::{Deserialize, Serialize};
use taggenator::taggenator::models::record::Record;

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
}

#[derive(Serialize)]
pub struct StartupFolder {
	pub location: String,
}

pub fn get_locations() -> std::io::Result<Vec<StartupFolder>> {
	let mut folders = vec![];

	let base = std::env::current_dir()?;
	folders.push(StartupFolder {
		location: base.to_string_lossy().to_string(),
	});

	return Ok(folders);
}
