import { promisified } from "tauri/api/tauri";
import { IRecord } from "./interfaces";

class Bridge {
	constructor() {
		// await promisified({
		// });
	}

	async getRecords(args: { args: string[] }): Promise<IRecord[]> {
		return helper("GetRecords", args);
	}

	async getTags(args: { args: string[] }): Promise<String[]> {
		return helper("GetTags", args);
	}

	// returns the tags we just added
	async addTags(args: { record: IRecord, tag_line: string }): Promise<IRecord> {
		return helper("AddTags", args);
	}

	async openRecord(args: { record: IRecord }): Promise<IRecord> {
		return helper("OpenRecord", args);
	}

	async getGrabBag(args: { record: IRecord }): Promise<{ [key: string]: string }> {
		return helper("GetGrabBag", args);
	}

	async getInitialArguments(): Promise<string[]> {
		return helper("GetInitialArguments", {});
	}

	async getPort(): Promise<number> {
		return helper("GetPort", {});
	}

}

function helper<T>(tagname: string, args: any): Promise<T> {
	return promisified({ cmd: tagname, ...args });
}

export const bridge = new Bridge();