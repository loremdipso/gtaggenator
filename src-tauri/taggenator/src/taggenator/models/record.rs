use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
	pub RecordID: i64,
	pub Name: String,
	pub Location: String,

	// TODO: maybe use vector for speed?
	// pub Tags: Vec<String>,
	pub Tags: HashSet<String>,

	pub Size: i64,
	pub Length: i32,
	pub TimesOpened: i32,

	pub DateAdded: Option<DateTime<Utc>>,
	pub DateCreated: Option<DateTime<Utc>>,
	pub DateLastAccessed: Option<DateTime<Utc>>,

	pub HaveManuallyTouched: bool,
	pub Imported: bool,
}

// TODO: probably we don't need to clone
#[derive(Debug, Clone)]
pub struct MiniRecord {
	pub RecordID: i64,
	pub Location: String,
}

pub const SQL: &str = "
	CREATE TABLE Records (
		RecordID INTEGER PRIMARY KEY,
		Name VARCHAR(500),
		Location VARCHAR(1000),

		Size BIGINT DEFAULT -1,
		Length INTEGER DEFAULT -1,
		TimesOpened INTEGER DEFAULT 0,

		DateAdded DateTime,
		DateCreated DateTime,
		DateLastAccessed DateTime,

		HaveManuallyTouched BOOLEAN DEFAULT 0,
		Imported BOOLEAN DEFAULT 0
	);
";
