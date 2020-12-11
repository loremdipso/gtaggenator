
export interface IRecord {
	RecordID: number,
	Name: string,
	Location: string,

	Tags: string[],

	Size: number,
	Length: number,
	TimesOpened: number,

	DateAdded: string,
	DateCreated: string,
	DateLastAccessed: string,

	HaveManuallyTouched: boolean,
}

export interface IStartupOptions {
	folders: IStartupFolder[]
	skip: boolean
}

export interface IStartupFolder {
	location: string
}