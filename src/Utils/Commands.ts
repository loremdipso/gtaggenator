import { promisified } from "tauri/api/tauri";
import { IRecord, IStartupOptions } from "./interfaces";

export enum CACHE_KEYS {
	search = "search",
	deltas = "deltas"
}

class Bridge {
	async getStartupOptions(): Promise<IStartupOptions> {
		return helper("GetStartupOptions", {});
	}

	async removeFolder(args: { path: string }): Promise<boolean> {
		return helper("RemoveFolder", args);
	}

	async initialize(args: { location: string }): Promise<never> {
		return helper("Initialize", args);
	}

	async openNewFolder(): Promise<boolean> {
		return helper("OpenNewFolder", {});
	}

	async reload(): Promise<never> {
		return helper("Reload", {});
	}

	async getRecommendedTags(args: { record: IRecord }): Promise<string[]> {
		return helper("GetRecommendedTags", args);
	}

	async editSettings(): Promise<IStartupOptions> {
		return helper("EditSettings", {});
	}

	async getRecords(args: { args: string[] }): Promise<IRecord[]> {
		return helper("GetRecords", args);
	}

	async getTags(): Promise<string[]> {
		let tags = await helper<string[]>("GetTags", {});

		// sort, excluding the prefixes
		tags.sort(sortTagHelper);
		return tags;
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

	async openContainingFolder(args: { location: string }): Promise<never> {
		return helper("OpenContainingFolder", args);
	}

	async openNatively(args: { location: string }): Promise<never> {
		return helper("OpenNatively", args);
	}

	async openAll(args: { locations: string[] }): Promise<never> {
		return helper("OpenAll", args);
	}

	async getCache(args: { key: CACHE_KEYS }): Promise<string> {
		return helper("GetCache", args);
	}

	async setCache(args: { key: CACHE_KEYS, value: string }): Promise<never> {
		return helper("SetCache", args);
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

// cut off the part before any ":"'s
function getTagBase(tag: string): string {
	return tag.substr(tag.indexOf(":") + 1);
}

function sortTagHelper(a: string, b: string): number {
	let aBase = getTagBase(a);
	let bBase = getTagBase(b);

	if (aBase === bBase) {
		return a.length - b.length;
	} else {
		return aBase.length - bBase.length;
	}
}

export const bridge = new Bridge();