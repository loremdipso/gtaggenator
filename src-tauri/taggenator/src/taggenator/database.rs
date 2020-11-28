// TODO: remove
#![allow(warnings, unused)]

use crate::taggenator::errors::BError;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;

static SETTINGS_FILENAME: &str = "tagg.db";

pub fn testDB() -> Result<(), BError> {
	// let db = MemoryDatabase::<HashMap<u32, String>, Ron>::memory(HashMap::new())?;
	// let db = FileDatabase::<HashMap<u32, String>, Ron>::memory(HashMap::new())?;
	let conn = Connection::open_with_flags(
		SETTINGS_FILENAME,
		OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
	)?;

	conn.execute_batch(
		"
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
        ",
	)
	.unwrap();

	// println!("Writing to Database");
	// db.write(|db| {
	// 	for i in 1..1_000_000 {
	// 		db.insert(i, format!("world: {}", i));
	// 	}
	// });

	// db.read(|db| {
	// 	// db.insert("foo".into(), String::from("bar"));
	// 	// The above line will not compile since we are only reading
	// 	println!("Hello: {:?}", db.get(&0));
	// })?;

	Ok(())
}
