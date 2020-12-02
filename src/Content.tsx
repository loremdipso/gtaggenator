import { useEffect, useState } from "react";
import { bridge } from "./Commands";
import { toast } from "react-toastify";
import { IRecord } from "./interfaces";

interface IContent {
	record: IRecord | null;
}
export function Content({ record }: IContent) {
	let [grabBag, setGrabBag] = useState({} as { [key: string]: string });
	useEffect(() => {
		getGrabBag();
	}, [record]);

	const getGrabBag = () => {
		if (record) {
			(async () => {
				// TODO: show this somewhere
				// also, should we actually load this all the time?
				try {
					let grabBag = await bridge.get_grab_bag({ record: record });
					setGrabBag(grabBag);
					console.log(grabBag);
				} catch (e) {
					console.log(e);
				}
			})();
		}
	};

	if (!record) {
		return null;
	}
	let path = getPath(record.Location);

	return (
		<div className="content-container">
			<button onClick={() => getGrabBag()}>DOIT</button>
			{Object.keys(grabBag).map((key) => (
				<div key={key}>
					<div>Key = {key}</div>
					<div>Value = {grabBag[key]}</div>
				</div>
			))}
			{isImage(record.Name) ? (
				<img alt="content" width={500} height={500} src={path} />
			) : null}

			{isVideo(record.Name) ? (
				<video controls autoPlay>
					<source src={path} type="video/mp4" />
					Your browser does not support the video tag.
				</video>
			) : null}
		</div>
	);
}

function getPath(path: string): string {
	path = path.substring(2); // remove the leading './'
	console.log(path);
	return `http://0.0.0.0:8000/static/${path}`;
}

function isImage(name: string): boolean {
	let extensions = ["jpg", "png", "gif"];
	let extension = getExtension(name);
	return !!extensions.find((e) => e === extension);
}

function isVideo(name: string): boolean {
	let extensions = ["mp4", "wmv", "avi", "m4v", "flv"];
	let extension = getExtension(name);
	return !!extensions.find((e) => e === extension);
}

function getExtension(name: string): string {
	let pieces = name.split(".");
	let extension = pieces[pieces.length - 1];
	return extension;
}
