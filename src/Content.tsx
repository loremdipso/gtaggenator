import "./App.css";
import { IRecord } from "./interfaces";

interface IContent {
	record: IRecord | null;
}
export function Content({ record }: IContent) {
	if (!record) {
		return null;
	}

	return <div>{record.Location}</div>;
}
