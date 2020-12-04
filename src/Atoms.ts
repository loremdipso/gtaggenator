import { atom } from "recoil";

export const currentRecordIndex = atom({
	key: 'currentRecordIndex',
	default: 0,
});