use std::sync::Arc;
use std::sync::Mutex;
use taggenator::taggenator::database::searcher::Searcher;
use taggenator::BError;
use taggenator::Taggenator;

mod cmd;

pub fn start_tauri(mut taggenator: Taggenator) -> Result<(), BError> {
	taggenator.update_files()?;
	let taggenator = Arc::new(Mutex::new(taggenator));

	tauri::AppBuilder::new()
		.invoke_handler(move |_webview, arg| {
			use cmd::Cmd::*;
			use cmd::*;
			match serde_json::from_str(arg) {
				Err(e) => Err(e.to_string()),
				Ok(command) => {
					match command {
						// sync
						DoSomethingSync { argument } => {
							//  your command code
							println!("{}", argument);
						}

						DoSomethingAsync {
							count,
							payload,
							callback,
							error,
						} => tauri::execute_promise(
							_webview,
							move || {
								dbg!(payload);
								if count > 5 {
									let response = Response {
										value: 5,
										message: "async response!",
									};
									Ok(response)
								} else {
									Err(CommandError::new("count should be > 5").into())
								}
							},
							callback,
							error,
						),

						AddTags {
							callback,
							error,
							recordId,
							tags,
						} => {
							let mut taggenator = taggenator.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator = taggenator.lock().unwrap();
									taggenator.database.add_tags(recordId, tags);
									return Ok(());
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
							let mut taggenator = taggenator.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let taggenator = taggenator.lock().unwrap();
									println!("{:?}", &args);
									let mut searcher = Searcher::new(args).unwrap(); // TODO: how do we bubble errors up?

									let records =
										searcher.get_records(&taggenator.database).unwrap();
									// println!("records: {:?}", records);
									return Ok(records);
								},
								callback,
								error,
							);
						}

						GetTags {
							callback,
							error,
							args,
						} => {
							let mut taggenator = taggenator.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let taggenator = taggenator.lock().unwrap();
									println!("{:?}", &args);
									let mut searcher = Searcher::new(args).unwrap(); // TODO: how do we bubble errors up?

									let tags = searcher.get_tags(&taggenator.database).unwrap();
									// println!("tags: {:?}", tags);
									return Ok(tags);
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
