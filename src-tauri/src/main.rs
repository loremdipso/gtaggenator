#![allow(dead_code)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
use std::env;
mod cli;
mod tauri;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	if args.len() == 0 {
		tauri::start_tauri()
	} else {
		cli::parse_args(args)
	}
}
