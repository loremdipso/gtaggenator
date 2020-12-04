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
						<ImageContainer path={getPath(record.Location)} />
					) : null}
					{isVideo(record.Name) ? (
						<VideoContainer record={record} />
					) : null}
					{isComic(record.Name) ? (
						<ComicContainer record={record} />
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
				// TODO: make sure this doesn't freak out and loop forever
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
	const containerRef = useRef(null as HTMLDivElement | null);
	const [volumeHeight, setVolumeHeight] = useState(0);
	const [durationWidth, setDurationWidth] = useState(0);
	const [muted, setMuted] = useState(false);

	useEffect(() => {
		let currentVideo = videoRef?.current;
		let currentContainer = containerRef?.current;
		if (currentVideo && currentContainer) {
			const updateTime = () => {
				// console.log("updating time");
				if (currentVideo && currentContainer) {
					setDurationWidth(
						(currentVideo.currentTime / currentVideo.duration) *
							currentContainer.clientWidth
					);
				}
			};
			const updateVolume = () => {
				// console.log("updating volume");
				if (currentVideo && currentContainer) {
					setVolumeHeight(
						(currentVideo.volume / 1.0) *
							currentContainer.clientHeight
					);
					setMuted(currentVideo.muted);
				}
			};
			const updateBoth = () => {
				updateTime();
				updateVolume();
			};

			currentVideo.addEventListener("timeupdate", updateTime);
			currentVideo.addEventListener("volumechange", updateVolume);
			currentContainer.addEventListener("resize", updateBoth);

			return () => {
				if (currentVideo) {
					currentVideo.removeEventListener("timeupdate", updateTime);
					currentVideo.removeEventListener(
						"volumechange",
						updateVolume
					);
				}
				if (currentContainer) {
					currentContainer.removeEventListener("resize", updateBoth);
				}
			};
		}
	}, [
		record,
		videoRef?.current,
		containerRef?.current,
		setVolumeHeight,
		setDurationWidth,
	]);

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
		return Math.max(0, duration - 30);
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

	useHotkeysHelper("alt+m", () => {
		if (videoRef?.current) {
			let current = videoRef.current;
			current.muted = !current.muted;
		}
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

	useHotkeysHelper("alt+i", () => {
		// toast("near middle");
		if (videoRef?.current) {
			let current = videoRef.current;
			current.currentTime = current.duration / 2;
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
		scrub(-times.LARGE);
	});
	useHotkeysHelper("alt+shift+k", () => {
		// toast("big forward");
		scrub(times.LARGE);
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
		<div className="video-container" ref={containerRef}>
			<video controls autoPlay ref={videoRef} src={path} />
			<div className="progress-bar" style={{ width: durationWidth }} />
			<div
				className={`volume-bar ${muted ? "muted" : ""}`}
				style={{ height: volumeHeight }}
			/>
		</div>
	);
}

interface IImageContainer {
	path: string;
}

function ImageContainer({ path }: IImageContainer) {
	const [fit, setFit] = useState("fit-best" as "fit-best" | "fit-width");
	useHotkeysHelper("alt+f", () => {
		setFit((fit) => {
			if (fit === "fit-best") {
				return "fit-width";
			} else {
				return "fit-best";
			}
		});
	});

	return (
		<div className={`image-container ${fit}`}>
			<img src={path} />
		</div>
	);
}

interface IComicContainer {
	record: IRecord;
}

function ComicContainer({ record }: IComicContainer) {
	useEffect(() => {
		(async () => {
			await getComicInfo(record.Location);
		})();
	}, []);
	let pageIndex = 0;
	let path = record ? getComicPagePath(record.Location, pageIndex) : "";
	return <ImageContainer path={path} />;
}

interface IComicInfo {
	num_pages: number;
}
async function getComicInfo(path: string): Promise<IComicInfo> {
	console.log("A");
	let response = await fetch(
		`http://0.0.0.0:8000/get_comic_info?path=${path}`
	);
	console.log("B");
	let info = await response.json();
	console.log("C");
	console.log(info);
	return info;
}

function getComicPagePath(path: string, pageIndex: number): string {
	return `http://0.0.0.0:8000/get_comic_page?path=${path}&page_number=${pageIndex}`;
}

function getPath(path: string): string {
	path = path.substring(2); // remove the leading './' (specific to how our file server works)
	return `http://0.0.0.0:8000/static/${path}`;
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
