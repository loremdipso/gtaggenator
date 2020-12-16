import React from "react";
import { IRecord } from "../Utils/interfaces";
import { readOnlySelector, useRecoilState } from "recoil";
import { fileServerPort } from "../Utils/Atoms";
import { ComicContainer } from "./Content/ComicContainer";
import { FlashContainer } from "./Content/FlashContainer";
import { GrabBag } from "./Content/GrabBag";
import { ImageContainer } from "./Content/ImageContainer";
import { VideoContainer } from "./Content/VideoContainer";

interface IContent {
	record: IRecord | null;
}
export function Content({ record }: IContent) {
	const [port] = useRecoilState(fileServerPort);
	if (!port) {
		return null;
	}

	return (
		<div className="content-container">
			{record ? (
				<>
					{isImage(record.Name) ? (
						<ImageContainer path={getPath(port, record.Location)} />
					) : null}
					{isVideo(record.Name) ? (
						<VideoContainer location={record.Location} />
					) : null}
					{isComic(record.Name) ? (
						<ComicContainer
							location={record.Location}
							recordId={record.RecordID}
						/>
					) : null}
					{isFlash(record.Name) ? (
						<FlashContainer path={getPath(port, record.Location)} />
					) : null}
					<GrabBag record={record} />
				</>
			) : null}
		</div>
	);
}

export function getPath(port: number, path: string): string {
	path = path.substring(2); // remove the leading './' (specific to how our file server works)
	path = encodeURIComponent(path);
	return `http://0.0.0.0:${port}/static/${path}`;
}

function isComic(name: string): boolean {
	let extensions = ["cbz", "cbr"];
	let extension = getExtension(name);
	return !!extensions.find((e) => e === extension);
}

function isImage(name: string): boolean {
	let extensions = ["jpg", "png", "gif"];
	let extension = getExtension(name);
	return !!extensions.find((e) => e === extension);
}

function isFlash(name: string): boolean {
	let extensions = ["swf"];
	let extension = getExtension(name);
	return !!extensions.find((e) => e === extension);
}

function isVideo(name: string): boolean {
	let extensions = ["mp4", "wmv", "avi", "m4v", "flv", "mkv"];
	let extension = getExtension(name);
	return !!extensions.find((e) => e === extension);
}

function getExtension(name: string): string {
	let pieces = name.split(".");
	let extension = pieces[pieces.length - 1];
	return extension;
}
