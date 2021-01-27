use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use crate::taggenator::utils::input::readline;
use crate::Taggenator;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf; // 0.8

pub fn open_all(taggenator: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	let mut searcher = Searcher::new(args)?;
	let mut records = searcher.get_records(&taggenator.database)?;

	open_all_core(
		records
			.iter()
			.map(|record| record.Location.clone())
			.collect(),
	);
	return Ok(());
}

pub fn open_all_core(locations: Vec<String>) -> Option<String> {
	let rand_filename: String = rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(10)
		.map(char::from)
		.collect();
	let filename = format!("/tmp/gt_{}.m3u", rand_filename);

	let mut out_file = File::create(&filename);
	if let Ok(mut file) = out_file {
		file.write_all(b"#EXTM3U\n");
		println!("Filename: {}", filename);
		for location in locations {
			let location = std::fs::canonicalize(PathBuf::from(location));
			if let Ok(location) = location {
				file.write_all(location.as_os_str().as_bytes());
				file.write_all(b"\n");
				// println!("{}", record.Location);
			}
		}
		return Some(filename);
	}

	return None;
}
