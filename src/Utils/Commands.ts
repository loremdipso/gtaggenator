import { FileEarmarkEasel } from "react-bootstrap-icons";
import { promisified } from "tauri/api/tauri";
import { IRecord, IStartupOptions } from "./interfaces";

class Bridge {
	constructor() {
		// await promisified({
		// });
	}

	async getStartupOptions(): Promise<IStartupOptions> {
		return helper("GetStartupOptions", {});
	}

	async initialize(args: { location: string }): Promise<IStartupOptions> {
		return helper("Initialize", args);
	}

	async openNewFolder(): Promise<IStartupOptions> {
		return helper("OpenNewFolder", {});
	}

	async reload(): Promise<IStartupOptions> {
		return helper("Reload", {});
	}

	async editSettings(): Promise<IStartupOptions> {
		return helper("EditSettings", {});
	}

	async getRecords(args: { args: string[] }): Promise<IRecord[]> {
		return helper("GetRecords", args);
	}

	async getTags(): Promise<string[]> {
		return helper("GetTags", {});
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

	async openContainingFolder(args: { location: string }): Promise<number> {
		return helper("OpenContainingFolder", args);
	}

	async openNatively(args: { location: string }): Promise<number> {
		return helper("OpenNatively", args);
	}
}

let DEBUG = !(window.__TAURI_INVOKE_HANDLER__);
function helper<T>(tagname: string, args: any): Promise<T> {
	if (DEBUG) {
		return fake(tagname, args);
	} else {
		return promisified({ cmd: tagname, ...args });
	}
}

function fake(tagname: string, args: any): Promise<any> {
	return new Promise((resolve) => {
		switch (tagname) {
			case "GetRecords":
				return resolve([fakeRecord(), fakeRecord()]);

			case "GetTags":
				{
					let tags: string[] = [];
					Array.from({ length: 1000 }).map((_, i) => tags.push(`tag ${i}`));
					return resolve(tags);
				}

			case "AddTags":
				{
					let record: IRecord = { ...args.record };
					let tags: string[] = (args.tag_line as string).split(",").map((e) => e.trim());
					for (let tag of tags) {
						if (record.Tags.indexOf(tag) === -1) {
							record.HaveManuallyTouched = true;
							record.Tags.push(tag);
						}
					}
					return resolve(record);
				}

			case "OpenRecord":
				{
					let record: IRecord = { ...args.record };
					record.TimesOpened += 1;
					return resolve(record);
				}

			case "GetGrabBag":
				return resolve({});

			case "GetInitialArguments":
				return resolve([]);

			case "GetPort":
				return resolve(4242);
		}
	});
}

function fakeRecord(): IRecord {
	return {
		RecordID: 1,
		Name: "name",
		Location: "location",

		Tags: ["some tag"],

		Size: 5,
		Length: 10,
		TimesOpened: 2,

		DateAdded: "",
		DateCreated: "",
		DateLastAccessed: "",

		HaveManuallyTouched: true,
	};
}

export const bridge = new Bridge();