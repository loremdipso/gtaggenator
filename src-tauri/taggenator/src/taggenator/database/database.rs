use crate::taggenator::database::writer::Query;
use crate::taggenator::database::writer::Sqlizable;
use crate::taggenator::database::writer::Sqlizable::Boolean;
use crate::taggenator::database::writer::Sqlizable::Date;
use crate::taggenator::database::writer::Sqlizable::Number;
use crate::taggenator::database::writer::Sqlizable::Text;
use crate::taggenator::database::writer::Writer;
use crate::taggenator::database::writer::MAX_BATCH_SIZE;
use crate::taggenator::errors::BError;
use crate::taggenator::models;
use crate::taggenator::models::record::MiniRecord;
use chrono::prelude::*;
use multimap::MultiMap;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
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
	sender: Sender<Option<Vec<Query>>>,
	writer: Writer,

	// the idea here is we want to wait until the write thread
	// has finished writing, so let's let us tell it how many things its
	// written and use that to decrement our todo count
	todo_receiver: Receiver<usize>,
	todo_count: i64,

	am_batching: bool,
	batch: Vec<Query>,
}

impl Database {
	pub fn new() -> Result<Database, BError> {
		// TODO: remove
		std::fs::remove_file(DATABASE_FILENAME);

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
			am_batching: false,
			batch: vec![],
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
		let size = 42;
		let length = 42;
		let times_opened = 0;
		let date_added = Some(Utc::now());
		let date_created = Some(Utc::now());
		let date_last_touched = None;
		let have_manually_touched = false;

		self.async_write(
			"
			INSERT INTO records (
				Name,
				Location,

				Size,
				Length,
				TimesOpened,

				DateAdded,
				DateCreated,
				DateLastAccessed,

				HaveManuallyTouched
			)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
			vec![
				Text(filename.to_string()),
				Text(location.to_string()),
				Number(size),
				Number(length),
				Number(times_opened),
				Date(date_added),
				Date(date_created),
				Date(date_last_touched),
				Boolean(have_manually_touched),
			],
		)
	}

	pub fn update_location(&mut self, recordID: i32, location: &str) -> Result<(), BError> {
		self.async_write(
			"UPDATE Records SET Location = ?
			WHERE Records.RecordID = ?",
			vec![Text(location.to_string()), Text(recordID.to_string())],
		)
	}

	pub fn get_filenames_to_locations(&self) -> Result<MultiMap<String, MiniRecord>, BError> {
		let mut stmt = self
			.conn
			.prepare("SELECT RecordID, Name, Location FROM Records")?;
		let mut rows = stmt.query(NO_PARAMS)?;

		let mut map: MultiMap<String, MiniRecord> = MultiMap::new();
		loop {
			let row = rows.next()?;
			match row {
				None => break,
				Some(row) => {
					let id = row.get(0)?;
					let name = row.get(1)?;
					let location = row.get(2)?;
					map.insert(
						name,
						MiniRecord {
							RecordID: id,
							Location: location,
						},
					);
				}
			}
		}
		Ok(map)
	}

	pub fn delete_record(&mut self, recordID: i32) -> Result<(), BError> {
		self.async_write(
			"DELETE FROM Records WHERE RecordID=?",
			vec![Number(recordID)],
		)
	}

	pub fn add_tag(&mut self, recordId: &str, tags: Vec<String>) -> Result<(), BError> {
		let mut args = vec![Text(recordId.to_string())];
		for tag in tags {
			args.push(Text(tag));
		}
		self.async_write("INSERT INTO Tags (RecordID, TagName) VALUES (?1, ?2)", args)
	}

	pub fn start_batch(&mut self) {
		self.am_batching = true;
	}

	pub fn end_batch(&mut self) {
		self.am_batching = false;
		self.send_batch();
	}

	fn async_write(&mut self, sql: &str, params: Vec<Sqlizable>) -> Result<(), BError> {
		let query = Query {
			sql: sql.to_string(),
			params: params,
		};

		self.batch.push(query);
		if !self.am_batching || self.batch.len() >= MAX_BATCH_SIZE {
			self.send_batch();
		}

		self.todo_count += 1;

		return Ok(());
	}

	fn send_batch(&mut self) -> Result<(), BError> {
		self.sender.send(Some(self.batch.drain(..).collect()))?;
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
