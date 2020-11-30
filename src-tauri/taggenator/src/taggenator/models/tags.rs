pub const SQL: &str = "
CREATE TABLE Tags (
	TagID INTEGER PRIMARY KEY,
	RecordID INTEGER,
	TagName VARCHAR(255),
	FOREIGN KEY(RecordID) REFERENCES Records(RecordID),
	UNIQUE(RecordID, TagName)
);
";
