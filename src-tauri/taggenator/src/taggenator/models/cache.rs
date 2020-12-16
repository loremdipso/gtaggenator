use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
	pub Key: String,
	pub Value: String,
}

pub const SQL: &str = "
	CREATE TABLE Cache (
		CacheID INTEGER PRIMARY KEY,
		Key VARCHAR(500),
		Value VARCHAR(5000),
		UNIQUE(Key)
	);
";
