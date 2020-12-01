pub const SQL: &str = "
CREATE TABLE GrabBag (
	GrabBagID INTEGER PRIMARY KEY,
	RecordID INTEGER,
	Key VARCHAR(255),
	Value VARCHAR(255),
	FOREIGN KEY(RecordID) REFERENCES Records(RecordID),
	UNIQUE(RecordID, Key)
);
";
