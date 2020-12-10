import React, { useCallback, useEffect, useRef, useState } from "react";
import { useRecoilState } from "recoil";
import { currentRecordIndex, fileServerPort } from "../../Utils/Atoms";
import { useHotkeysHelper } from "../../Utils/Hotkeys";
import { IRecord } from "../../Utils/interfaces";
import { ImageContainer } from "./ImageContainer";

interface IComicContainer {
	record: IRecord;
}

const NUM_TO_PRELOAD = 5;
const NUM_MAX_IMAGES = 20;
export function ComicContainer({ record }: IComicContainer) {
	const [recordIndex, setRecordIndex] = useRecoilState(currentRecordIndex);
	const [pageIndex, setPageIndex] = useState(0);
	const [comicInfo, setComicInfo] = useState(null as IComicInfo | null);
	const [images, setImages] = useState([] as number[]);

	const preload = useCallback(
		async (newIndex: number) => {
			if (comicInfo) {
				let newImages = [...images];
				let thisBatch = [];
				let didSet = false;
				for (let i = newIndex - 1; i < newIndex + NUM_TO_PRELOAD; i++) {
					if (
						i >= 0 &&
						i < comicInfo.pages.length &&
						images.indexOf(i) < 0
					) {
						newImages.push(i);
						didSet = true;
					}

					thisBatch.push(i);
				}

				if (didSet) {
					newImages = newImages.filter(
						(value) =>
							Math.abs(value - newIndex) < NUM_MAX_IMAGES / 2
					);
					setImages(newImages);
				}
			}
		},
		[comicInfo, images]
	);

	const updatePage = useCallback(
		(newIndex: number) => {
			if (comicInfo) {
				if (newIndex >= 0 && newIndex < comicInfo.pages.length) {
					setPageIndex(newIndex);
					preload(newIndex);
				}

				if (newIndex >= comicInfo.pages.length) {
					setRecordIndex(recordIndex + 1);
				} else if (newIndex < 0 && recordIndex > 0) {
					setRecordIndex(recordIndex - 1);
				}
			}
		},
		[setRecordIndex, setPageIndex, comicInfo, preload, recordIndex]
	);

	const [port] = useRecoilState(fileServerPort);

	const nextPage = () => {
		updatePage(pageIndex + 1);
	};
	const previousPage = () => {
		updatePage(pageIndex - 1);
	};

	useEffect(() => {
		(async () => {
			let info = await getComicInfo(port, record.Location);
			setComicInfo(info);
		})();
	}, [record, setComicInfo]);

	useEffect(() => {
		updatePage(0);
	}, [comicInfo]);

	useHotkeysHelper(
		"alt+h",
		() => {
			updatePage(0);
		},
		[updatePage]
	);
	useHotkeysHelper(
		"home",
		() => {
			updatePage(0);
		},
		[updatePage]
	);
	useHotkeysHelper(
		"alt+i",
		() => {
			if (comicInfo) {
				updatePage(Math.round(comicInfo.pages.length / 2));
			}
		},
		[updatePage, comicInfo]
	);
	useHotkeysHelper(
		"alt+j",
		() => {
			previousPage();
		},
		[previousPage]
	);
	useHotkeysHelper(
		"pageup",
		() => {
			previousPage();
		},
		[previousPage]
	);
	useHotkeysHelper(
		"alt+k",
		() => {
			nextPage();
		},
		[nextPage]
	);
	useHotkeysHelper(
		"pagedown",
		() => {
			nextPage();
		},
		[nextPage]
	);
	useHotkeysHelper(
		"alt+l",
		() => {
			if (comicInfo) {
				updatePage(comicInfo.pages.length - 1);
			}
		},
		[updatePage, comicInfo]
	);
	useHotkeysHelper(
		"end",
		() => {
			if (comicInfo) {
				updatePage(comicInfo.pages.length - 1);
			}
		},
		[updatePage, comicInfo]
	);

	return (
		<div
			className="image-container"
			onClick={(event: React.MouseEvent) => {
				let target = event.target as HTMLDivElement;
				if (event.pageX > target.clientWidth / 2 + target.offsetLeft) {
					nextPage();
				} else {
					previousPage();
				}
			}}
			onWheel={(event: React.WheelEvent) => {
				if ((event as any).deltaY > 0) {
					nextPage();
				} else {
					previousPage();
				}
			}}
		>
			{record && comicInfo ? (
				<>
					<div
						style={{
							position: "absolute",
							top: 0,
							right: 0,
							backgroundColor: "black",
						}}
					>
						{pageIndex + 1}/{comicInfo.pages.length}
					</div>

					{images.map((imageIndex) => (
						<ImageContainer
							key={imageIndex}
							path={getComicPagePath(
								port,
								record.Location,
								comicInfo.pages[imageIndex]
							)}
							hidden={imageIndex !== pageIndex}
						/>
					))}
				</>
			) : null}
		</div>
	);
}

interface IComicInfo {
	pages: number[];
}
async function getComicInfo(port: number, path: string): Promise<IComicInfo> {
	let response = await fetch(
		`http://0.0.0.0:${port}/get_comic_info?path=${path}`
	);
	let info = await response.json();
	return info;
}

function getComicPagePath(
	port: number,
	path: string,
	pageIndex: number
): string {
	return `http://0.0.0.0:${port}/get_comic_page?path=${path}&page_number=${pageIndex}`;
}
