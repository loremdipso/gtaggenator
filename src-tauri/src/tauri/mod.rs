use crate::Taggenator;

mod cmd;

pub fn start_tauri(taggenator: Taggenator) {
	tauri::AppBuilder::new()
		.invoke_handler(|_webview, arg| {
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

						GetTags { callback, error } => tauri::execute_promise(
							_webview,
							move || Ok(vec!["A", "B"]),
							callback,
							error,
						),
					}
					Ok(())
				}
			}
		})
		.build()
		.run();
}
