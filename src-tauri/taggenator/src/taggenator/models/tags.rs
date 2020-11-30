pub const SQL: &str = "
CREATE TABLE Tags (
	TagID INTEGER PRIMARY KEY,
	RecordID INTEGER,
	TagName VARCHAR(255),
	FOREIGN KEY(RecordID) REFERENCES Records(RecordID),
	UNIQUE(RecordID, TagName)
);

CREATE VIRTUAL TABLE TagsFTS USING fts5(
	RecordID,
	TagName,
	content='Tags'
);

CREATE TRIGGER Record_ai AFTER INSERT ON Tags
    BEGIN
        INSERT INTO TagsFTS (RecordID, TagName)
        VALUES (new.RecordID, new.TagName);
    END;

CREATE TRIGGER Record_ad AFTER DELETE ON Tags
    BEGIN
        INSERT INTO TagsFTS (TagsFTS, RecordID, TagName)
        VALUES ('delete', old.id, old.TagName);
    END;

CREATE TRIGGER Record_au AFTER UPDATE ON Tags
    BEGIN
        INSERT INTO TagsFTS (TagsFTS, RecordID, TagName)
        VALUES ('delete', old.id, old.TagName);
        INSERT INTO TagsFTS (RecordID, TagName)
        VALUES (new.id, new.TagName);
    END;
";
