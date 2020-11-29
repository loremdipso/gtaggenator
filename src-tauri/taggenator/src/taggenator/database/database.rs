// TODO: remove
#![allow(warnings, unused)]

use crate::taggenator::database::writer::Query;
use crate::taggenator::database::writer::Writer;
use crate::taggenator::errors::BError;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

static SETTINGS_FILENAME: &str = "tagg.db";
static END_OF_WRITES: &str = "end";

pub struct Database {
	conn: Connection,
	sender: Sender<Option<Query>>,
	writer: Writer,
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

		let (sender, receiver) = channel();
		let writer = Writer::new(sender.clone(), receiver)?;
		// sender.send("A".to_string());
		sender.send(Some(Query {
			sql: "INSERT INTO foo VALUES (?)".to_string(),
			params: vec!["woop".to_string()],
		}));

		return Ok(Database {
			conn: conn,
			sender: sender,
			writer: writer,
		});
	}

	pub fn testRead(&self) -> Result<Vec<i32>, BError> {
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
			match tx.execute("INSERT INTO foo VALUES (?)", &[&"sup"]) {
				Ok(_) => (),
				Err(err) => println!("update failed: {}", err),
			}
		}

		tx.commit()?;
		Ok(())
	}
}
