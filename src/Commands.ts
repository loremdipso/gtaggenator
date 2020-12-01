import { promisified } from "tauri/api/tauri";

class Bridge {
	constructor() {
		// await promisified({
		// });
	}

	async add_tags(tags: string[]): Promise<never> {
		return promisified({
			cmd: "AddTags",
			tags
		});
	}

	async get_tags(args: string[]): Promise<String[]> {
		return promisified({
			cmd: "GetTags",
			args
		});
	}
}

export const bridge = new Bridge();