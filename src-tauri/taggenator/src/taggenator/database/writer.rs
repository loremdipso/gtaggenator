use crate::taggenator::database::END_OF_WRITES;
use crate::taggenator::database::SETTINGS_FILENAME;
use crate::taggenator::errors::BError;
use rusqlite::ToSql;
use rusqlite::{Connection, OpenFlags};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

pub const MAX_BATCH_SIZE: usize = 1000; // TODO: what should this number be?

pub struct Query {
	pub sql: String,
	pub params: Vec<String>,
}

pub struct Writer {
	sender: Sender<Option<Vec<Query>>>,
	worker: Option<JoinHandle<()>>,
}

impl Drop for Writer {
	fn drop(&mut self) {
		self.sender.send(None);

		// this is called the "option dance",
		// necessary since after we join the writer is an invalid reference. Neat!
		self.worker.take().unwrap().join();
	}
}

impl Writer {
	pub fn new(
		sender: Sender<Option<Vec<Query>>>,
		receiver: Receiver<Option<Vec<Query>>>,
		todo_sender: Sender<usize>,
	) -> Result<Writer, BError> {
		let mut conn = Connection::open_with_flags(
			SETTINGS_FILENAME,
			OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
		)?;

		let worker = thread::spawn(move || loop {
			match receiver.recv() {
				Ok(value) => {
					let mut should_return = false;
					let mut batch: Vec<Query> = vec![];

					match value {
						None => return,
						Some(actual_value) => batch.extend(actual_value),
					}

					while let Ok(value) = receiver.try_recv() {
						match value {
							None => should_return = true,
							Some(actual_value) => batch.extend(actual_value),
						}
					}

					if let Err(e) = Writer::write_batch(&mut conn, &batch) {
						dbg!(e);
					}

					// TOOD: this could get out of sync if there's an error
					todo_sender.send(batch.len());

					if should_return {
						return;
					}
				}
				Err(e) => {
					dbg!(e);
				}
			}
		});

		Ok(Writer {
			worker: Some(worker),
			sender: sender,
		})
	}

	fn write_batch(conn: &mut Connection, batch: &Vec<Query>) -> Result<(), BError> {
		for chunk in batch.chunks(MAX_BATCH_SIZE) {
			let tx = conn.transaction()?;
			for action in chunk {
				match tx.execute(&action.sql, &action.params) {
					Ok(_) => (),
					Err(err) => println!("update failed: {}", err),
				}
			}
			tx.commit()?;
		}
		Ok(())
	}
}
