// TODO: remove
#![allow(warnings, unused)]

use crate::taggenator::errors::BError;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::path::Path;

static SETTINGS_FILENAME: &str = "tagg.db";

pub struct Database {
	conn: Connection,
}

impl Database {
	pub fn new() -> Result<Database, BError> {
		let didExist = Path::new(SETTINGS_FILENAME).exists();

		let mut conn = Connection::open_with_flags(
			SETTINGS_FILENAME,
			OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
		)?;

		if !didExist {
			conn.execute_batch(
				"BEGIN;
			CREATE TABLE foo(x INTEGER);
			CREATE TABLE bar(y TEXT);
			COMMIT;",
			)?;
		}

		return Ok(Database { conn: conn });
	}

	pub fn testRead(&self) -> Result<Vec<i32>, BError> {
		// for i in 1..count {
		let mut stmt = self.conn.prepare("SELECT * FROM foo")?;
		let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

		let mut names = Vec::new();
		for name_result in rows {
			names.push(name_result?);
		}
		Ok(names)
	}

	pub fn testWrite(&mut self, count: i32) -> Result<(), BError> {
		// match conn.execute("UPDATE foo SET bar = 'baz' WHERE qux = ?", &[&1i32]) {
		let tx = self.conn.transaction()?;

		for i in 1..count {
			match tx.execute("INSERT INTO foo VALUES (?)", &[&i]) {
				Ok(_) => (),
				Err(err) => println!("update failed: {}", err),
			}
		}

		tx.commit()?;
		Ok(())
	}
}
