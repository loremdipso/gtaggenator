import { useEffect, useRef, useState } from "react";
import { useRecoilState } from "recoil";
import { fileServerPort } from "../../Utils/Atoms";
import { useHotkeysHelper } from "../../Utils/Hotkeys";
import { IRecord } from "../../Utils/interfaces";
import { getPath } from "../Content";

const times = {
	TINY: 5,
	SMALL: 10,
	MEDIUM: 60,
	LARGE: 60 * 5,
};

interface IVideoContainer {
	location: string;
}
export function VideoContainer({ location }: IVideoContainer) {
	const videoRef = useRef(null as HTMLVideoElement | null);
	const containerRef = useRef(null as HTMLDivElement | null);
	const [volumeHeight, setVolumeHeight] = useState(0);
	const [durationWidth, setDurationWidth] = useState(0);
	const [muted, setMuted] = useState(false);

	const [port] = useRecoilState(fileServerPort);

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
	}, [videoRef, containerRef, setVolumeHeight, setDurationWidth]);

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
		scrub(-times.SMALL);
	});
	useHotkeysHelper("alt+k", () => {
		// forward
		scrub(times.SMALL);
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
	useHotkeysHelper("alt+numpad1", () => {
		// small back
		scrub(-times.SMALL);
	});
	useHotkeysHelper("alt+numpad2", () => {
		volumeDown();
	});
	useHotkeysHelper("alt+numpad3", () => {
		// small forward
		scrub(times.SMALL);
	});
	useHotkeysHelper("alt+numpad4", () => {
		// back
		scrub(-times.MEDIUM);
	});
	useHotkeysHelper("alt+numpad6", () => {
		// forward
		scrub(times.MEDIUM);
	});
	useHotkeysHelper("alt+numpad7", () => {
		// toast("big back");
		scrub(-times.LARGE);
	});
	useHotkeysHelper("alt+numpad8", () => {
		volumeUp();
	});
	useHotkeysHelper("alt+numpad9", () => {
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

	let path = location ? getPath(port, location) : "";
	return (
		<div className="video-container" ref={containerRef}>
			<video controls loop autoPlay ref={videoRef} src={path} />
			<div className="progress-bar" style={{ width: durationWidth }} />
			<div
				className={`volume-bar ${muted ? "muted" : ""}`}
				style={{ height: volumeHeight }}
			/>
		</div>
	);
}
