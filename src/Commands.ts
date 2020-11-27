import { promisified } from "tauri/api/tauri";

class Bridge {
	constructor() {
		// await promisified({
		// });
	}

	async get_tags(): Promise<String[]> {
		return promisified({
			cmd: "GetTags"
		});
	}
}

export const bridge = new Bridge();