#[derive(Debug)]
pub struct Record {
	pub RecordID: i32,
	pub Name: String,
	pub Location: String,

	pub Tags: Vec<String>,

	pub Size: i32,
	pub Length: i32,
	pub TimesOpened: i32,

	pub DateAdded: Option<String>,
	pub DateCreated: Option<String>,
	pub DateLastAccessed: Option<String>,

	pub HaveManuallyTouched: bool,
}

// TODO: probably we don't need to clone
#[derive(Debug, Clone)]
pub struct MiniRecord {
	pub RecordID: i32,
	pub Location: String,
}

pub const SQL: &str = "
	CREATE TABLE Records (
		RecordID INTEGER PRIMARY KEY,
		Name VARCHAR(500),
		Location VARCHAR(1000),

		Size INTEGER DEFAULT -1,
		Length INTEGER DEFAULT -1,
		TimesOpened INTEGER DEFAULT 0,

		DateAdded String,
		DateCreated String,
		DateLastAccessed String,

		HaveManuallyTouched BOOLEAN
	);
";
