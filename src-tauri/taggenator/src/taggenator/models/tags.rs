pub const SQL: &str = "
CREATE TABLE Tags (
	TagID INTEGER PRIMARY KEY,
	RecordID INTEGER,
	TagName VARCHAR(255),
	DateAdded DateTime,
	FOREIGN KEY(RecordID) REFERENCES Records(RecordID) ON DELETE CASCADE,
	UNIQUE(RecordID, TagName)
);
";
