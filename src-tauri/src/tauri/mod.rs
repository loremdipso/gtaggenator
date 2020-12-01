use std::sync::Arc;
use std::sync::Mutex;
use taggenator::errors::MyCustomError::UnknownError;
use taggenator::taggenator::database::searcher::Searcher;
use taggenator::BError;
use taggenator::Taggenator;
use warp::Filter;

mod cmd;

pub fn start_tauri(mut taggenator: Taggenator) -> Result<(), BError> {
	// start file server in separate thread
	std::thread::spawn(move || {
		serve_fs();
	});

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
							mut record,
							tag_line,
						} => {
							let mut taggenator = taggenator.clone();
							tauri::execute_promise(
								_webview,
								move || {
									let mut taggenator =
										taggenator.lock().map_err(|_| UnknownError)?;
									taggenator
										.insert_tag_line(&mut record, tag_line)
										.map_err(|_| UnknownError)?;
									return Ok(record);
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
									let taggenator = taggenator.lock().map_err(|_| UnknownError)?;
									println!("{:?}", &args);
									let mut searcher =
										Searcher::new(args).map_err(|_| UnknownError)?;

									let records = searcher
										.get_records(&taggenator.database)
										.map_err(|_| UnknownError)?;
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

#[tokio::main]
async fn serve_fs() {
	// NOTE: this seems to work fine, but should we use actix-web instead?
	let route = warp::path("static").and(warp::fs::dir("."));
	let route = route.with(warp::log("warp-server"));
	warp::serve(route).run(([0, 0, 0, 0], 8000)).await;
}
