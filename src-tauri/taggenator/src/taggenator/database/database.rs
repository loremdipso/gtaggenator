use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::writer::Query;
use crate::taggenator::database::writer::Sqlizable;
use crate::taggenator::database::writer::Sqlizable::Boolean;
use crate::taggenator::database::writer::Sqlizable::Date;
use crate::taggenator::database::writer::Sqlizable::Number;
use crate::taggenator::database::writer::Sqlizable::Text;
use crate::taggenator::database::writer::Writer;
use crate::taggenator::database::writer::MAX_BATCH_SIZE;
use crate::taggenator::database::DATABASE_FILENAME;
use crate::taggenator::errors::BError;
use crate::taggenator::errors::MyCustomError;
use crate::taggenator::errors::MyCustomError::UnknownError;
use crate::taggenator::models;
use crate::taggenator::models::record::MiniRecord;
use crate::taggenator::models::record::Record;
use crate::taggenator::tag_recommender::TagRecommender;
use chrono::prelude::*;
use multimap::MultiMap;
use pathdiff::diff_paths;
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, OpenFlags};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

pub struct Database {
	pub conn: Connection,
	sender: Sender<Option<Vec<Query>>>,
	writer: Writer,

	// the idea here is we want to wait until the write thread
	// has finished writing, so let's let us tell it how many things its
	// written and use that to decrement our todo count
	todo_receiver: Receiver<usize>,
	todo_count: i64,

	batching_count: i32,
	batch: Vec<Query>,

	recommender: Option<TagRecommender>,
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
				"BEGIN; {} {} {} COMMIT;",
				models::record::SQL,
				models::tags::SQL,
				models::grabbag::SQL
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
			batching_count: 0,
			batch: vec![],
			recommender: None,
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
		let metadata = fs::metadata(location)?;

		let size = metadata.len() as i64;
		let length = -1; // TODO: fetch the length of video files
		let times_opened = 0;
		let date_added = Some(Utc::now());

		let mut date_created = None;
		if let Ok(created) = metadata.created() {
			date_created = Some(DateTime::<Utc>::from(created));
		}

		let date_last_touched = None;
		let have_manually_touched = false;
		let was_imported = false;
		self.add_record_core(
			filename.to_string(),
			location.to_string(),
			size,
			length,
			times_opened,
			date_added,
			date_created,
			date_last_touched,
			have_manually_touched,
			was_imported,
		)
	}

	pub fn add_record_core(
		&mut self,
		filename: String,
		location: String,
		size: i64,
		length: i64,
		times_opened: i64,
		date_added: Option<DateTime<chrono::Utc>>,
		date_created: Option<DateTime<chrono::Utc>>,
		date_last_touched: Option<DateTime<chrono::Utc>>,
		have_manually_touched: bool,
		was_imported: bool,
	) -> Result<(), BError> {
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

				HaveManuallyTouched,
				Imported
			)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
				Boolean(was_imported),
			],
		)
	}

	pub fn add_record_by_location(
		&mut self,
		filename: String,
		location: String,
		size: i64,
		length: i64,
		times_opened: i64,
		date_added: Option<DateTime<chrono::Utc>>,
		date_created: Option<DateTime<chrono::Utc>>,
		date_last_touched: Option<DateTime<chrono::Utc>>,
		have_manually_touched: bool,
		was_imported: bool,
	) -> Result<(), BError> {
		let location = self.get_location_relative_to_base(&location)?;

		// set to now if we don't have this
		let date_added = if date_added.is_none() {
			Some(Utc::now())
		} else {
			date_added
		};

		self.async_write(
			"UPDATE records
			SET
				Name = ?,
				Location = ?,

				Size = ?,
				Length = ?,
				TimesOpened = ?,

				DateAdded = ?,
				DateCreated = ?,
				DateLastAccessed = ?,

				HaveManuallyTouched = ?,
				Imported = ?
			WHERE Records.Location = ?",
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
				Boolean(was_imported),
				Text(location.to_string()),
			],
		)
	}

	pub fn update_location(&mut self, recordID: i64, location: &str) -> Result<(), BError> {
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

	pub fn delete_record(&mut self, recordID: i64) -> Result<(), BError> {
		self.async_write(
			"DELETE FROM Records WHERE RecordID=?",
			vec![Number(recordID)],
		)
	}

	pub fn add_tags(&mut self, recordId: i64, tags: &Vec<String>) -> Result<(), BError> {
		self.start_batch();
		for tag in tags {
			self.add_tag(recordId, tag)?;
		}
		self.end_batch();
		Ok(())
	}

	pub fn add_tag(&mut self, recordId: i64, tag: &String) -> Result<(), BError> {
		if self.recommender.is_some() {
			self.recommender.as_mut().unwrap().add_tag(tag);
		}

		self.async_write(
			"INSERT INTO Tags (RecordID, TagName, DateAdded) VALUES (?1, ?2, ?3)",
			vec![
				Number(recordId),
				Text(tag.to_lowercase().trim().to_string()),
				Date(Some(Utc::now())),
			],
		)
	}

	pub fn add_tag_by_location(&mut self, location: String, tag: String) -> Result<(), BError> {
		let location = self.get_location_relative_to_base(&location)?;
		println!("{}", location);
		self.async_write(
			"INSERT INTO Tags (RecordID, TagName, DateAdded)
			SELECT RecordID, ?, ?
			FROM Records WHERE Records.Location = ?",
			vec![Text(tag), Date(Some(Utc::now())), Text(location)],
		)
	}

	pub fn grabbag_get(&mut self, recordId: i64, key: String) -> Result<String, BError> {
		return Ok(self.conn.query_row(
			"SELECT Value FROM grabbag WHERE RecordID = ? AND Key = ?",
			params![&recordId, &key],
			|row| row.get(0),
		)?);
	}
	pub fn grabbag_get_by_location(
		&mut self,
		location: String,
		key: String,
	) -> Result<String, BError> {
		let location = self.get_location_relative_to_base(&location)?;
		return Ok(self.conn.query_row(
			"SELECT Value FROM grabbag WHERE Key = ?
			AND EXISTS(
				SELECT Records.RecordID FROM Records
				WHERE Records.Location = ?
				AND Records.RecordID = grabbag.RecordID 
			)",
			params![&key, &location],
			|row| row.get(0),
		)?);
	}

	pub fn grabbag_get_all(&mut self, recordId: i64) -> Result<HashMap<String, String>, BError> {
		let mut stmt = self
			.conn
			.prepare("SELECT Key, Value FROM grabbag WHERE grabbag.RecordID = ?")?;
		let mut rows = stmt.query(params![&recordId])?;

		let mut map: HashMap<String, String> = HashMap::new();
		loop {
			let row = rows.next()?;
			if let Some(row) = row {
				let key = row.get(0)?;
				let value = row.get(1)?;
				map.insert(key, value);
			} else {
				return Ok(map);
			}
		}
	}
	pub fn grabbag_get_all_by_location(
		&mut self,
		location: String,
	) -> Result<HashMap<String, String>, BError> {
		let location = self.get_location_relative_to_base(&location)?;
		let mut stmt = self.conn.prepare(
			"SELECT Key, Value, RecordID FROM grabbag
			WHERE EXISTS(
				SELECT Records.RecordID FROM Records
				WHERE Records.Location = ?
				AND Records.RecordID = grabbag.RecordID 
			)",
		)?;
		let mut rows = stmt.query(params![&location])?;

		let mut map: HashMap<String, String> = HashMap::new();
		loop {
			let row = rows.next()?;
			if let Some(row) = row {
				let key = row.get(0)?;
				let value = row.get(1)?;
				map.insert(key, value);
			} else {
				return Ok(map);
			}
		}
	}

	pub fn grabbag_upsert(
		&mut self,
		recordId: i64,
		key: String,
		value: String,
	) -> Result<(), BError> {
		self.async_write(
			"REPLACE INTO grabbag (RecordID, Key, Value) VALUES(?, ?, ?)",
			vec![Number(recordId), Text(key), Text(value)],
		)
	}
	pub fn grabbag_upsert_by_location(
		&mut self,
		location: String,
		key: String,
		value: String,
	) -> Result<(), BError> {
		let location = self.get_location_relative_to_base(&location)?;
		println!("{}", location);
		self.async_write(
			"REPLACE INTO grabbag (RecordID, Key, Value)
			SELECT RecordID, ?, ?
			FROM Records WHERE Records.Location = ?",
			vec![Text(key), Text(value), Text(location)],
		)
	}

	pub fn grabbag_delete(&mut self, recordId: i64, key: String) -> Result<(), BError> {
		self.async_write(
			"DELETE FROM grabbag WHERE RecordID = ?1 AND Key = ?2",
			vec![Number(recordId), Text(key)],
		)
	}
	pub fn grabbag_delete_by_location(
		&mut self,
		location: String,
		key: String,
	) -> Result<(), BError> {
		let location = self.get_location_relative_to_base(&location)?;
		self.async_write(
			"DELETE FROM grabbag WHERE Location = ?1 AND Key = ?2",
			vec![Text(location), Text(key)],
		)
	}

	pub fn set_times_opened(&mut self, recordId: i64, times_opened: i32) -> Result<(), BError> {
		self.async_write(
			"UPDATE Records SET TimesOpened = ?
			WHERE Records.RecordID = ?",
			vec![Number(times_opened.into()), Number(recordId)],
		)
	}

	pub fn set_touched(&mut self, recordId: i64, touched: bool) -> Result<(), BError> {
		self.async_write(
			"UPDATE Records SET HaveManuallyTouched = ?
			WHERE Records.RecordID = ?",
			vec![Boolean(touched), Number(recordId)],
		)
	}

	pub fn remove_tags(&mut self, recordId: i64, tags: &Vec<String>) -> Result<(), BError> {
		self.start_batch();
		for tag in tags {
			self.remove_tag(recordId, &tag)?;
		}
		self.end_batch();
		Ok(())
	}

	pub fn remove_tag(&mut self, recordId: i64, tag: &String) -> Result<(), BError> {
		self.async_write(
			"DELETE FROM Tags WHERE RecordID = ?1 AND TagName = ?2",
			vec![Number(recordId), Text(tag.to_string())],
		)
	}

	pub fn start_batch(&mut self) {
		self.batching_count += 1;
	}

	pub fn end_batch(&mut self) {
		self.batching_count -= 1;
		if self.batching_count <= 0 {
			self.send_batch();
		}
	}

	fn async_write(&mut self, sql: &str, params: Vec<Sqlizable>) -> Result<(), BError> {
		let query = Query {
			sql: sql.to_string(),
			params: params,
		};

		self.batch.push(query);
		if self.batching_count == 0 || self.batch.len() >= MAX_BATCH_SIZE {
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

	fn get_location_relative_to_base(&self, location: &String) -> Result<String, BError> {
		if location.chars().nth(0).unwrap_or_default() == '.' {
			return Ok(location.clone());
		}

		let mut base = std::env::current_dir()?; // TODO: perf, calculate once
		let mut diff = diff_paths(Path::new(location), base).unwrap();
		let mut real = PathBuf::new();
		real.push(".");
		real.push(diff);

		return Ok(real.to_string_lossy().to_string());
	}

	// TODO: this is bad encapsulation. Fix it
	pub fn get_all_tags(&mut self) -> HashSet<String> {
		let mut searcher = Searcher::new(vec![]).unwrap();
		let tags = searcher.get_tags(&self).unwrap();
		self.recommender = Some(TagRecommender::new(tags.iter()));
		return tags;
	}

	pub fn get_recommended_tags(&mut self, record: &Record) -> Vec<String> {
		if self.recommender.is_none() {
			self.get_all_tags();
		}

		return self
			.recommender
			.as_ref()
			.unwrap()
			.recommend(&record.Location, &record.Tags);
	}
}
