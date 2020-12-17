import { atom } from "recoil";

export const fileServerPort = atom({
	key: 'fileServerPort',
	default: 0,
});

export const currentRecordIndex = atom({
	key: 'currentRecordIndex',
	default: 0,
});

export const allTagsAtom = atom({
	key: 'allTags',
	default: [] as string[],
});

export const currentTagAtom = atom({
	key: 'currentTag',
	default: "" as string,
});