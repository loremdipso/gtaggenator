import { promisified } from "tauri/api/tauri";
import { addEmitHelper } from "typescript";
import { IRecord } from "./interfaces";

class Bridge {
	constructor() {
		// await promisified({
		// });
	}

	async get_records(args: { args: string[] }): Promise<IRecord[]> {
		return helper("GetRecords", args);
	}

	async get_tags(args: { args: string[] }): Promise<String[]> {
		return helper("GetTags", args);
	}

	// returns the tags we just added
	async add_tags(args: { record: IRecord, tag_line: string }): Promise<IRecord> {
		return helper("AddTags", args);
	}
}

function helper<T>(tagname: string, args: any): Promise<T> {
	return promisified({ cmd: tagname, ...args });
}

export const bridge = new Bridge();