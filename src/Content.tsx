import { useEffect, useState } from "react";
import { bridge } from "./Commands";
import { toast } from "react-toastify";
import { IRecord } from "./interfaces";
import { useHotkeysHelper } from "./Utils";

interface IContent {
	record: IRecord | null;
}
export function Content({ record }: IContent) {
	return (
		<div className="content-container">
			{record ? (
				<>
					<GrabBag record={record} />
					{isImage(record.Name) ? (
						<ImageContainer record={record} />
					) : null}
					{isVideo(record.Name) ? (
						<VideoContainer record={record} />
					) : null}
				</>
			) : null}
		</div>
	);
}

interface IGrabBag {
	record: IRecord;
}
function GrabBag({ record }: IGrabBag) {
	let [grabBag, setGrabBag] = useState({} as { [key: string]: string });

	const getGrabBag = () => {
		if (record) {
			(async () => {
				console.log("sup");
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

	useEffect(() => {
		getGrabBag();
	}, [record]);

	return (
		<>
			{/* <button onClick={() => getGrabBag()}>DOIT</button> */}
			{Object.keys(grabBag).map((key) => (
				<div key={key}>
					<div>Key = {key}</div>
					<div>Value = {grabBag[key]}</div>
				</div>
			))}
		</>
	);
}

interface IVideoContainer {
	record: IRecord;
}
function VideoContainer({ record }: IVideoContainer) {
	const volumeUp = () => {
		toast("volume up");
	};

	const volumeDown = () => {
		toast("volume down");
	};

	useHotkeysHelper("alt+numpad8", () => {
		volumeUp();
	});
	useHotkeysHelper("alt+numpad2", () => {
		volumeDown();
	});

	useHotkeysHelper("alt+k", () => {
		toast("forward");
	});
	useHotkeysHelper("alt+j", () => {
		toast("back");
	});

	useHotkeysHelper("alt+l", () => {
		toast("near end");
	});

	useHotkeysHelper("alt+shift+j", () => {
		toast("big back");
	});
	useHotkeysHelper("alt+shift+k", () => {
		toast("big forward");
	});

	useHotkeysHelper("alt+numpad0", () => {
		toast("tiny back");
	});
	useHotkeysHelper("alt+numpad.", () => {
		toast("tiny forward");
	});

	let path = record ? getPath(record.Location) : "";
	return (
		<video controls autoPlay>
			<source src={path} type="video/mp4" />
			Your browser does not support the video tag.
		</video>
	);
}

interface IImageContainer {
	record: IRecord;
}
function ImageContainer({ record }: IImageContainer) {
	let path = record ? getPath(record.Location) : "";
	return <img alt="content" width={500} height={500} src={path} />;
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
