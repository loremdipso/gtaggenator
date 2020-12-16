use super::file_server;
use crate::take_flag;
use crate::tauri::cmd::CommandError;
use crate::tauri::cmd::Response;
use crate::tauri::cmd::StartupOptions;
use crate::tauri::cmd::{add_to_cache, get_locations};
use portpicker::pick_unused_port;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use taggenator::errors::MyCustomError::UnknownError;
use taggenator::taggenator::database::searcher::Searcher;
use taggenator::taggenator::SETTINGS_FILENAME;
use taggenator::BError;
use taggenator::Taggenator;
use tauri_api::dialog::Response::Okay;

pub fn start_tauri(mut args: Vec<String>) -> Result<(), BError> {
	// start file server in separate thread
	let file_server_port: u16 = pick_unused_port().expect("No ports free");
	std::thread::spawn(move || {
		file_server::serve_fs(file_server_port);
	});

	println!("Using port {}", file_server_port);

	let mut ignore_updates = false;
	if take_flag(&mut args, "--ignore-update") || take_flag(&mut args, "--stale") {
		println!("Ignore file system changes...");
		ignore_updates = true;
	}

	start_tauri_core(file_server_port, args, ignore_updates)?;

	return Ok(());
}

pub fn start_tauri_core(
	port: u16,
	initial_arguments: Vec<String>,
	ignore_updates: bool,
) -> Result<(), BError> {
	let do_skip_initialization: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
	let taggenator_box: Arc<Mutex<Option<Taggenator>>> = Arc::new(Mutex::new(None));

	// if opening with command line arguments, skip initial folder selection step
	if initial_arguments.len() > 0 {
		let do_skip_initialization = do_skip_initialization.clone();
		let mut do_skip_initialization = do_skip_initialization.lock().unwrap();
		*do_skip_initialization = true;

		let mut taggenator_box = taggenator_box.clone();
		let mut taggenator_option = taggenator_box.lock().map_err(|_| UnknownError)?;
		let location = std::env::current_dir().unwrap();
		let taggenator =
			initialize(ignore_updates, location.to_string_lossy().to_string()).unwrap();
		*taggenator_option = Some(taggenator);
	}

	tauri::AppBuilder::new()
		.invoke_handler(move |_webview, arg| {
			use super::cmd::Cmd::*;
			match serde_json::from_str(arg) {
				Err(e) => Err(e.to_string()),
				Ok(command) => {
					match command {
						GetStartupOptions { callback, error } => {
							let do_skip_initialization = do_skip_initialization.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut do_skip_initialization =
										do_skip_initialization.lock().unwrap();
									let do_skip = *do_skip_initialization;
									*do_skip_initialization = false;
									return Ok(StartupOptions {
										folders: get_locations().unwrap(),
										skip: do_skip,
									});
								},
								callback,
								error,
							);
						}

						Initialize {
							callback,
							error,
							location,
						} => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator_option =
										taggenator_box.lock().map_err(|_| UnknownError)?;
									let taggenator = initialize(ignore_updates, location).unwrap();
									*taggenator_option = Some(taggenator);
									return Ok(());
								},
								callback,
								error,
							);
						}

						Reload { callback, error } => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let location = std::env::current_dir().unwrap();
									let mut taggenator_option =
										taggenator_box.lock().map_err(|_| UnknownError)?;

									// this time we definitely want to query for updates
									let taggenator =
										initialize(false, location.to_string_lossy().to_string())
											.unwrap();
									*taggenator_option = Some(taggenator);
									return Ok(());
								},
								callback,
								error,
							);
						}

						OpenNewFolder { callback, error } => {
							let mut taggenator_box = taggenator_box.clone();
							let response = tauri_api::dialog::pick_folder(Some(
								std::env::current_dir().unwrap(),
							))
							.unwrap();

							tauri::execute_promise(
								_webview,
								move || {
									if let Okay(location) = response {
										let mut taggenator_option =
											taggenator_box.lock().map_err(|_| UnknownError)?;
										let taggenator =
											initialize(ignore_updates, location).unwrap();
										*taggenator_option = Some(taggenator);
										return Ok(());
									} else {
										panic!();
									}
								},
								callback,
								error,
							);
						}

						// sync
						DoSomethingSync { argument } => {
							//  your command code
							println!("{}", argument);
						}

						AddTags {
							callback,
							error,
							mut record,
							tag_line,
						} => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										taggenator
											.insert_tag_line(&mut record, tag_line)
											.map_err(|_| UnknownError)?;
										return Ok(record);
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						}

						GetRecords {
							callback,
							error,
							args,
						} => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									println!("{:?}", &args);
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										let mut searcher =
											Searcher::new(args).map_err(|_| UnknownError)?;

										let records = searcher
											.get_records(&taggenator.database)
											.map_err(|_| UnknownError)?;

										return Ok(records);
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						}

						GetTags { callback, error } => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										let tags = taggenator.database.get_all_tags();
										return Ok(tags);
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						}

						OpenRecord {
							callback,
							error,
							mut record,
						} => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										record.TimesOpened += 1;
										taggenator
											.database
											.set_times_opened(record.RecordID, record.TimesOpened)
											.map_err(|_| UnknownError)?;
										taggenator
											.database
											.set_last_opened_to_now(record.RecordID)
											.map_err(|_| UnknownError)?;
										return Ok(record);
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						}

						GetGrabBag {
							callback,
							error,
							mut record,
						} => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										let grab_bag = taggenator
											.database
											.grabbag_get_all(record.RecordID)
											.map_err(|_| UnknownError)?;
										return Ok(grab_bag);
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						}

						SetGrabBagKey {
							callback,
							error,
							mut record,
							key,
							value,
						} => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										let grab_bag = taggenator
											.database
											.grabbag_upsert(record.RecordID, key, value)
											.map_err(|_| UnknownError)?;
										return Ok(());
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						}

						GetRecommendedTags {
							callback,
							error,
							mut record,
						} => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										let tags =
											taggenator.database.get_recommended_tags(&record);
										return Ok(tags);
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						}

						GetPort { callback, error } => {
							tauri::execute_promise(
								_webview,
								move || {
									return Ok(port);
								},
								callback,
								error,
							);
						}

						GetInitialArguments { callback, error } => {
							let args = initial_arguments.clone();
							tauri::execute_promise(
								_webview,
								move || {
									return Ok(args);
								},
								callback,
								error,
							);
						}

						OpenContainingFolder {
							callback,
							error,
							location,
						} => {
							tauri::execute_promise(
								_webview,
								move || {
									// TODO: make generic
									// TODO: make path absolute
									// TODO: disown
									Command::new("nemo").arg(location).spawn();
									return Ok(());
								},
								callback,
								error,
							);
						}

						OpenNatively {
							callback,
							error,
							location,
						} => {
							tauri::execute_promise(
								_webview,
								move || {
									// TODO: make generic
									// TODO: make path absolute
									// TODO: disown
									Command::new("xdg-open").arg(location).spawn();
									return Ok(());
								},
								callback,
								error,
							);
						}

						EditSettings { callback, error } => {
							tauri::execute_promise(
								_webview,
								move || {
									// TODO: make generic
									// TODO: make path absolute
									// TODO: disown
									let path = std::env::current_dir().unwrap();
									let location = path.join(SETTINGS_FILENAME);
									Command::new("code").arg(location).spawn();
									return Ok(());
								},
								callback,
								error,
							);
						}

						GetCache { callback, error, key } => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										let cache = taggenator.database.get_cache(key).map_err(|_| UnknownError)?;
										return Ok(cache);
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);
						},

						SetCache { callback, error, key, value } => {
							let mut taggenator_box = taggenator_box.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator_box.lock().unwrap();
									if let Some(ref mut taggenator) = *taggenator {
										taggenator.database.set_cache(key, value).map_err(|_| UnknownError);
										return Ok(());
									} else {
										// TODO: remove, throw error
										panic!();
									}
								},
								callback,
								error,
							);

						}
					}
					Ok(())
				}
			}
		})
		.build()
		.run();

	Ok(())
}

fn initialize(ignore_updates: bool, location: String) -> Result<Taggenator, BError> {
	std::env::set_current_dir(&location)?;

	let mut taggenator = Taggenator::new_headless().unwrap();
	if !ignore_updates {
		taggenator.update_files().map_err(|_| UnknownError)?;
	}

	add_to_cache(location);
	return Ok(taggenator);
}
