import { useEffect, useRef, useState } from "react";
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

const times = {
	TINY: 5,
	SMALL: 10,
	MEDIUM: 60,
	LARGE: 60 * 5,
};

interface IVideoContainer {
	record: IRecord;
}
function VideoContainer({ record }: IVideoContainer) {
	const videoRef = useRef(null as HTMLVideoElement | null);

	const volumeUp = () => {
		if (videoRef?.current) {
			let current = videoRef.current;
			current.volume = Math.min(1.0, current.volume + 0.05);
		}
	};

	const volumeDown = () => {
		if (videoRef?.current) {
			let current = videoRef.current;
			current.volume = Math.max(0.0, current.volume - 0.05);
		}
	};

	const getNearEnd = (duration: number) => {
		return duration - 30;
	};

	const scrub = (delta: number) => {
		if (videoRef?.current) {
			let current = videoRef.current;

			let duration = current.duration;
			let targetTime = current.currentTime + delta;
			if (targetTime > duration) {
				targetTime = duration;
			} else if (targetTime < 0) {
				targetTime = 0;
			}

			current.currentTime = targetTime;
		}
	};

	useHotkeysHelper("alt+numpad8", () => {
		volumeUp();
	});
	useHotkeysHelper("alt+numpad2", () => {
		volumeDown();
	});

	useHotkeysHelper("alt+space", () => {
		if (videoRef?.current) {
			let current = videoRef.current;
			if (current.paused) {
				current.play();
			} else {
				current.pause();
			}
		}
	});

	useHotkeysHelper("alt+h", () => {
		// toast("beginning");
		if (videoRef?.current) {
			videoRef.current.currentTime = 0;
		}
	});
	useHotkeysHelper("alt+j", () => {
		// back
		scrub(-times.MEDIUM);
	});
	useHotkeysHelper("alt+k", () => {
		// forward
		scrub(times.MEDIUM);
	});
	useHotkeysHelper("alt+l", () => {
		// toast("near end");
		if (videoRef?.current) {
			let current = videoRef.current;
			current.currentTime = getNearEnd(current.duration);
		}
	});

	useHotkeysHelper("alt+shift+j", () => {
		// toast("big back");
		scrub(times.LARGE);
	});
	useHotkeysHelper("alt+shift+k", () => {
		// toast("big forward");
		scrub(-times.LARGE);
	});

	useHotkeysHelper("alt+numpad0", () => {
		// toast("tiny back");
		scrub(-times.TINY);
	});
	useHotkeysHelper("alt+numpad.", () => {
		// toast("tiny forward");
		scrub(times.TINY);
	});

	let path = record ? getPath(record.Location) : "";
	return (
		<video controls autoPlay ref={videoRef}>
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
