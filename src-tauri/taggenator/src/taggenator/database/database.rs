use crate::taggenator::database::writer::Query;
use crate::taggenator::database::writer::Writer;
use crate::taggenator::errors::BError;
use crate::taggenator::models;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

static DATABASE_FILENAME: &str = "tagg.db";

pub struct Database {
	pub conn: Connection,
	sender: Sender<Option<Query>>,
	writer: Writer,

	// the idea here is we want to wait until the write thread
	// has finished writing, so let's let us tell it how many things its
	// written and use that to decrement our todo count
	todo_receiver: Receiver<usize>,
	todo_count: i64,
}

impl Database {
	pub fn new() -> Result<Database, BError> {
		// TODO: remove
		// std::fs::remove_file(DATABASE_FILENAME);

		let did_exist = Path::new(DATABASE_FILENAME).exists();

		let conn = Connection::open_with_flags(
			DATABASE_FILENAME,
			OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
		)?;

		if !did_exist {
			conn.execute_batch(&format!(
				"BEGIN; {} {} COMMIT;",
				models::record::SQL,
				models::tags::SQL
			))?;
		}

		let (sender, receiver) = channel();
		let (todo_sender, todo_receiver) = channel();

		let writer = Writer::new(sender.clone(), receiver, todo_sender)?;

		return Ok(Database {
			conn: conn,
			sender: sender,
			writer: writer,
			todo_receiver: todo_receiver,
			todo_count: 0,
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
			match tx.execute(
				"INSERT INTO records (Name) VALUES (?)",
				&[&format!("file_{}", i)],
			) {
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

	pub fn add_record(&mut self, filename: &str, location: &str) -> Result<(), BError> {
		self.async_write(
			"INSERT INTO records (Name, Location) VALUES (?, ?)",
			vec![filename.to_string(), location.to_string()],
		)
	}

	pub fn get_filenames(&self) -> Result<HashSet<String>, BError> {
		let mut stmt = self.conn.prepare("SELECT (Name) FROM Records")?;
		let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

		let mut names = HashSet::new();
		for name_result in rows {
			names.insert(name_result?);
		}
		Ok(names)
	}

	pub fn delete_record(&mut self, recordName: &str) -> Result<(), BError> {
		self.async_write(
			"DELETE FROM Records WHERE Name=?",
			vec![recordName.to_string()],
		)
	}

	pub fn add_tag(&mut self, recordId: &str, tags: Vec<String>) -> Result<(), BError> {
		let mut args = vec![recordId.to_string()];
		args.extend(tags);
		self.async_write("INSERT INTO Tags (RecordID, TagName) VALUES (?1, ?2)", args)
	}

	fn async_write(&mut self, sql: &str, params: Vec<String>) -> Result<(), BError> {
		self.sender.send(Some(Query {
			sql: sql.to_string(),
			params: params,
		}))?;

		self.todo_count += 1;

		return Ok(());
	}

	pub fn flush_writes(&mut self) -> Result<(), BError> {
		while self.todo_count > 0 {
			let value = self.todo_receiver.recv()?;
			self.todo_count -= value as i64;
		}
		Ok(())
	}
}
