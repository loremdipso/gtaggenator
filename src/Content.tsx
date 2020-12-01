import { useEffect } from "react";
import { getDefaultLibFilePath } from "typescript";
import "./App.css";
import { IRecord } from "./interfaces";

interface IContent {
	record: IRecord | null;
}
export function Content({ record }: IContent) {
	if (!record) {
		return null;
	}

	return (
		<div>
			{record.Location}
			<img width={500} height={500} src={getPath(record.Location)} />
		</div>
	);
}

function getPath(path: string): string {
	let pieces = path.split("/");
	// return `localhost:8000/${pieces[pieces.length - 1]}`;
	return `http://0.0.0.0:8000/test.jpg`;
}
