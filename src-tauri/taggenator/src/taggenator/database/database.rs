// TODO: remove
#![allow(warnings, unused)]

use crate::taggenator::database::writer::Query;
use crate::taggenator::database::writer::Writer;
use crate::taggenator::errors::BError;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

static SETTINGS_FILENAME: &str = "tagg.db";

pub struct Database {
	conn: Connection,
	sender: Sender<Option<Query>>,
	writer: Writer,
}

impl Database {
	pub fn new() -> Result<Database, BError> {
		let did_exist = Path::new(SETTINGS_FILENAME).exists();

		let conn = Connection::open_with_flags(
			SETTINGS_FILENAME,
			OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
		)?;

		if !did_exist {
			conn.execute_batch(
				"BEGIN;
			CREATE TABLE foo(x INTEGER);
			CREATE TABLE bar(y TEXT);
			COMMIT;",
			)?;
		}

		let (sender, receiver) = channel();
		let writer = Writer::new(sender.clone(), receiver)?;

		return Ok(Database {
			conn: conn,
			sender: sender,
			writer: writer,
		});
	}

	pub fn test_read(&self) -> Result<Vec<i32>, BError> {
		let mut stmt = self.conn.prepare("SELECT * FROM foo")?;
		let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

		let mut names = Vec::new();
		for name_result in rows {
			names.push(name_result?);
		}
		Ok(names)
	}

	pub fn test_write(&mut self, count: i32) -> Result<(), BError> {
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

	pub fn async_write_test(&self) -> Result<(), BError> {
		return Ok(());
	}

	fn async_write(&self, sql: &str, params: Vec<String>) -> Result<(), BError> {
		self.sender.send(Some(Query {
			sql: sql.to_string(),
			params: params,
		}))?;

		return Ok(());
	}
}
