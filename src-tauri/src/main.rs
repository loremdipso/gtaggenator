// TODO: remove
#![allow(dead_code, warnings, unused)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
use std::env;
use taggenator::flags::take_flag;
use taggenator::Taggenator;
mod tauri;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_json;


fn main() {
	let mut args: Vec<String> = env::args().skip(1).collect();
	let do_gopen = take_flag(&mut args, "gopen");

	if do_gopen || args.len() == 0 {
		tauri::start_tauri(args);
	} else {
		let mut taggenator = Taggenator::new().unwrap();
		if let Err(error) = taggenator.parse_args(args) {
			dbg!(error);
		}
	}
}
