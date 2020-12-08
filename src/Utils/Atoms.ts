import { atom } from "recoil";

export const fileServerPort = atom({
	key: 'fileServerPort',
	default: 0,
});

export const currentRecordIndex = atom({
	key: 'currentRecordIndex',
	default: 0,
});