pub const SQL: &str = "
CREATE TABLE Tags (
	RecordID INTEGER,
	TagName VARCHAR(255),
	primary key (RecordID, TagName),
	FOREIGN KEY(RecordID) REFERENCES Records(RecordID)
);
";
