#[derive(Debug)]
pub struct Record {
	Name: String,
}

pub const SQL: &str = "
	CREATE TABLE Records (
		RecordID INTEGER PRIMARY KEY,
		Name VARCHAR(500),
		Location VARCHAR(1000),
		Size INTEGER DEFAULT -1,
		Length INTEGER DEFAULT -1,
		TimesOpened INTEGER DEFAULT -1
	);
";
