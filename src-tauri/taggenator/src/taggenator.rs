// TODO: remove
#![allow(warnings, unused)]
use jwalk;
use std::error::Error;
use walkdir;

pub struct Taggenator {}

impl Taggenator {
	pub fn new() -> Taggenator {
		return Taggenator {};
	}

	pub fn parse_args(self, args: Vec<String>) -> Result<(), Box<Error>> {
		// dbg!(args);
		// dbg!(env::current_dir());
		self.base();
		// self.jwalk();
		Ok(())
	}

	fn jwalk(self) -> Result<(), Box<Error>> {
		let mut num_chars = 0;
		for entry in jwalk::WalkDir::new(".") {
			if let Some(name) = entry?.file_name().to_str() {
				num_chars += name.len();
			}
		}
		dbg!(num_chars);
		Ok(())
	}

	fn base(self) -> Result<(), Box<Error>> {
		let mut num_chars = 0;
		for entry in walkdir::WalkDir::new(".") {
			if let Some(name) = entry?.file_name().to_str() {
				num_chars += name.len();
			}
		}
		dbg!(num_chars);
		Ok(())
	}

	/// Attempt to load and parse the config file into our Config struct.
	/// If a file cannot be found, return a default Config.
	/// If we find a file but cannot parse it, panic
	pub fn parse(path: String) {
		// pub fn parse(path: String) -> Config {
		// let mut config_toml = String::new();

		// let mut file = match File::open(&path) {
		// 	Ok(file) => file,
		// 	Err(_) => {
		// 		error!("Could not find config file, using default!");
		// 		return Config::new();
		// 	}
		// };

		// file.read_to_string(&mut config_toml)
		// 	.unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

		// let mut parser = Parser::new(&config_toml);
		// let toml = parser.parse();

		// if toml.is_none() {
		// 	for err in &parser.errors {
		// 		let (loline, locol) = parser.to_linecol(err.lo);
		// 		let (hiline, hicol) = parser.to_linecol(err.hi);
		// 		println!(
		// 			"{}:{}:{}-{}:{} error: {}",
		// 			path, loline, locol, hiline, hicol, err.desc
		// 		);
		// 	}
		// 	panic!("Exiting server");
		// }

		// let config = Value::Table(toml.unwrap());
		// match toml::decode(config) {
		// 	Some(t) => t,
		// 	None => panic!("Error while deserializing config"),
		// }
	}
}
