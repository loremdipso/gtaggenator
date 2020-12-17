import React, { useEffect, useState } from "react";

import "./App.scss";
import { bridge, CACHE_KEYS } from "./Utils/Commands";
import { IRecord } from "./Utils/interfaces";
import { ArrowClockwise, ArrowRight, ArrowLeft } from "react-bootstrap-icons";
import { Content } from "./Components/Content";
import {
	IDelta,
	createDelta,
	DisplayDeltas,
	DisplayTagLineGroup,
	appendDeltaImmutable,
	fixDeltas,
} from "./Components/Deltas";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

import {
	Button,
	Jumbotron,
	OverlayTrigger,
	Spinner,
	Tab,
	Tabs,
} from "react-bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";

import DisplayRecord from "./Components/DisplayRecord";
import { useHotkeys } from "react-hotkeys-hook";

import ResizablePanel from "./Components/ResizablePanel";
import { useHotkeysHelper } from "./Utils/Hotkeys";
import { DisplayFilters, IFilter } from "./Components/Filters";
import Drawer from "./Components/Drawer";
import { RecoilRoot, useRecoilState } from "recoil";
import { allTagsAtom, currentRecordIndex, fileServerPort } from "./Utils/Atoms";
import { SpecialInput, SpecialInputSimple } from "./Components/SpecialInput";
import { SimpleTooltip } from "./Components/SimpleTooltip";
import { Initializer } from "./Components/Initializer";

import { setTitle } from "tauri/api/window";
import { addSearch, DisplaySearches, ISearch } from "./Components/Searches";

type ITabKey = "search" | "play" | "edit_settings";
const MAX_FILTERS = 20;

function App() {
	const [initialized, setInitialized] = useState(false);

	if (!initialized) {
		return <Initializer onInitialize={setInitialized} />;
	}

	return (
		<RecoilRoot>
			<AppContent setInitialized={setInitialized} />
		</RecoilRoot>
	);
}

interface IAppContent {
	setInitialized: (initialized: boolean) => any;
}

function AppContent({ setInitialized }: IAppContent) {
	const [args, setArgs] = useState([] as string[]);

	const [searchFocusEpoch, setSearchFocusEpoch] = useState(0);
	const [tagFocusEpoch, setTagFocusEpoch] = useState(0);

	const [tagLine, setTagLine] = useState("");
	const [search, setSearch] = useState("");

	const [searches, setSearches] = useState([] as ISearch[]);

	const [records, setRecords] = useState([] as IRecord[]);

	const [filters, setFilters] = useState([] as IFilter[]);
	const [deltas, setDeltas] = useState([] as IDelta[]);

	const [lastExecutedSearch, setLastExecutedSearch] = useState("");

	const [recordIndex, setRecordIndex] = useRecoilState(currentRecordIndex);
	const [currentRecord, setCurrentRecord] = useState(null as IRecord | null);
	const [allTags, setAllTags] = useRecoilState(allTagsAtom);

	const [tabKey, setTabKey] = useState("search" as ITabKey);

	const [recommendedTags, setRecommendedTags] = useState([] as string[]);

	const [lastOpenedRecordID, setLastOpenedRecordID] = useState(
		null as number | null
	);

	const [lastPushedDeltas, setLastPushedDeltas] = useState(deltas);
	useEffect(() => {
		if (lastPushedDeltas !== deltas) {
			console.log("About to update cache");
			const id = setTimeout(() => {
				console.log("Updating cache");
				bridge.setCache({
					key: CACHE_KEYS.deltas,
					value: JSON.stringify(deltas),
				});
			}, 2000);
			return () => {
				clearTimeout(id);
				console.log("Killing cache update");
			};
		}
	}, [deltas, lastPushedDeltas]);

	useEffect(() => {
		(async () => {
			// TODO: this
			try {
				let deltas_s = await bridge.getCache({
					key: CACHE_KEYS.deltas,
				});

				let deltas = JSON.parse(deltas_s) as IDelta[];
				fixDeltas(deltas);
				setDeltas(deltas);
				setLastPushedDeltas(deltas);
			} catch {
				// none found, likely
			}
		})();
	}, []);

	const [lastPushedSearches, setLastPushedSearches] = useState(searches);
	useEffect(() => {
		if (lastPushedSearches !== searches) {
			console.log("About to update search cache");
			const id = setTimeout(async () => {
				console.log("Updating search cache");
				await bridge.setCache({
					key: CACHE_KEYS.search,
					value: JSON.stringify(searches),
				});
			}, 2000);
			return () => {
				clearTimeout(id);
				console.log("Killing search cache update");
			};
		}
	}, [searches, lastPushedSearches]);

	const [isLoadingSearches, setIsLoadingSearches] = useState(false);
	useEffect(() => {
		(async () => {
			// TODO: this
			try {
				setIsLoadingSearches(true);
				let search_s = await bridge.getCache({
					key: CACHE_KEYS.search,
				});

				let searches = JSON.parse(search_s) as ISearch[];
				setSearches(searches);
				setLastPushedSearches(searches);
			} catch {
				// none found, likely
			}

			setIsLoadingSearches(false);
		})();
	}, []);

	const [, setPort] = useRecoilState(fileServerPort);
	useEffect(() => {
		(async () => {
			let port = await bridge.getPort();
			console.log(`Using port ${port}`);
			setPort(port);
		})();
	}, [setPort]);

	useEffect(() => {
		(async () => {
			try {
				let searches = await bridge.getCache({
					key: CACHE_KEYS.search,
				});
				console.log(searches);
				// TODO: this
				// setSearch();
			} catch {
				// none found, likely
			}
		})();
	}, []);

	const updateTabKey = (key: ITabKey | null) => {
		if (!key || tabKey === key) {
			return;
		}

		if (key === "search") {
			setSearchFocusEpoch((epoch) => epoch + 1);
		} else if (key === "play") {
			setTagFocusEpoch((epoch) => epoch + 1);
		}

		setTabKey(key);
	};

	const nextRecord = () => {
		if (records.length) {
			if (recordIndex >= records.length) {
				doEnd();
			} else {
				setRecordIndex(recordIndex + 1);
			}
		}
	};

	const previousRecord = () => {
		if (records.length) {
			setRecordIndex(Math.max(0, recordIndex - 1));
		}
	};

	const addAllRecommended = () => {
		addTagLine(recommendedTags.join(", "));
		setRecommendedTags([]);
	};

	useHotkeys("alt+a", () => addAllRecommended(), [addAllRecommended]);

	useHotkeys(
		"ctrl+j",
		() => {
			previousRecord();
		},
		[previousRecord]
	);

	useHotkeys(
		"ctrl+k",
		() => {
			nextRecord();
		},
		[nextRecord]
	);

	useHotkeysHelper(
		"alt+numpad*",
		() => {
			nextRecord();
		},
		[nextRecord]
	);

	useHotkeysHelper(
		"alt+numpad/",
		() => {
			previousRecord();
		},
		[previousRecord]
	);

	useHotkeysHelper(
		"alt+s",
		() => {
			setTabKey("search");
			setSearchFocusEpoch((epoch) => epoch + 1);
		},
		[setTabKey, setSearchFocusEpoch]
	);

	useHotkeysHelper(
		"alt+p",
		() => {
			setTabKey("play");
			setTagFocusEpoch((epoch) => epoch + 1);
		},
		[setTabKey, setSearchFocusEpoch]
	);

	// TODO: is this the most react-y thing we can do?
	const loadData = async (override?: string) => {
		let tempSearch = getSearch(search, filters, override);

		try {
			addSearch(setSearches, searches, tempSearch);

			let finalSearch = parseWords(tempSearch);
			let records = await bridge.getRecords({
				// split by whitespace, but keep quoted groups together
				args: finalSearch,
			});
			setRecords(records);

			let newIndex = 0;
			if (lastExecutedSearch === tempSearch && currentRecord) {
				// special case: if we're refreshing, try to find the record we were just on
				newIndex = records.findIndex(
					(record) => record.RecordID === currentRecord.RecordID
				);
				if (newIndex < 0) {
					newIndex = 0;
				}

				toast("Reloaded");
			} else {
				toast("Loaded");
			}
			setTabKey("play");
			setRecordIndex(newIndex);
			setTagFocusEpoch((epoch) => epoch + 1);
			setLastExecutedSearch(tempSearch);
		} catch (e) {
			console.log(e);
		}
	};

	// load initial arguments, once
	useEffect(() => {
		(async () => {
			let initialArgs = await bridge.getInitialArguments();
			setArgs(initialArgs);
		})();
	}, []);

	useEffect(() => {
		if (args.length) {
			setArgs([]);
			console.log(`initial args: ${args}`);
			let argsString = args.map((e) => `"${e}"`).join(" ");
			setSearch(argsString);
			// TODO: populate filters from this
			loadData(argsString);
		}
	}, [args, loadData]);

	useEffect(() => {
		if (currentRecord && currentRecord.RecordID !== lastOpenedRecordID) {
			(async () => {
				// we've got to keep track of this, otherwise we get an infinite loop, which is no bueno
				setLastOpenedRecordID(currentRecord.RecordID);
				// UI minor change: we'll update the record immediately, even before the request is finished
				updateRecord(setRecords, {
					...currentRecord,
					TimesOpened: currentRecord.TimesOpened + 1,
				});
				await bridge.openRecord({
					record: currentRecord,
				});

				setTitle(currentRecord.Name);
			})();

			setRecommendedTags([]);
			(async () => {
				let recommendedTags = await bridge.getRecommendedTags({
					record: currentRecord,
				});
				setRecommendedTags(recommendedTags);
			})();
		}
	}, [currentRecord, lastOpenedRecordID, setRecords, setRecommendedTags]);

	useEffect(() => {
		if (records.length === 0) {
			if (currentRecord) {
				setRecordIndex(0);
				setCurrentRecord(null);
			}
		} else if (recordIndex < 0) {
			setRecordIndex(0);
		} else if (recordIndex >= records.length) {
			// TODO: exit?
			doEnd();
		} else if (records.length > recordIndex) {
			setCurrentRecord(records[recordIndex]);
		}
	}, [recordIndex, records, currentRecord, setRecordIndex]);

	useEffect(() => {
		(async () => {
			let tags = await bridge.getTags();
			setAllTags(tags);
		})();
	}, [setAllTags]);

	const doEnd = () => {
		setRecords([]);
		setTabKey("search");
		setSearchFocusEpoch((oldEpoch) => oldEpoch + 1);
	};

	const addTagLine = (tag: string) => {
		addTags(tag, true);
	};

	const removeTagLine = (tag: string) => {
		// TODO: handle multiple tags
		addTags(`-${tag}`, true);
	};

	const addTags = async (line: string, ignoreDelta: boolean = false) => {
		if (!currentRecord) {
			return;
		}

		let oldRecord = currentRecord;
		let newRecord = await bridge.addTags({
			record: currentRecord,
			tag_line: line,
		});

		if (!ignoreDelta) {
			let delta = createDelta(newRecord.Tags, oldRecord.Tags, line);
			if (delta.added.length || delta.removed.length) {
				// insert after all our favorite deltas
				setDeltas((deltas) =>
					appendDeltaImmutable(deltas, delta).slice(0, MAX_FILTERS)
				);
			}
		}

		// add net-new tags to our global tags
		let tagsToAdd = difference(newRecord.Tags, allTags);
		if (tagsToAdd.length) {
			setAllTags((allTags) => [...allTags, ...tagsToAdd]);
		}

		let addedTags = difference(newRecord.Tags, oldRecord.Tags);
		for (let tag of addedTags) {
			toast(tag, {
				position: "bottom-right",
				className: "green-toast",
			});
		}

		let removedTags = difference(oldRecord.Tags, newRecord.Tags);
		for (let tag of removedTags) {
			toast(tag, { position: "bottom-right", className: "red-toast" });
		}

		newRecord.Tags = sortTags(newRecord.Tags, oldRecord.Tags);

		updateRecord(setRecords, newRecord);
	};

	const undoAdds = (delta: IDelta) => {
		let line = "";
		for (let added of delta.added) {
			if (line.length > 0) {
				line += ", ";
			}
			line += `-${added}`;
		}

		if (line.length > 0) {
			addTags(line, true);
		}
	};

	// const reload = () => {
	// 	loadData();
	// };

	// const editSelf = () => {
	// 	// TODO: this
	// };

	// const removeSelf = () => {
	// 	// TODO: this
	// };

	const clearTags = () => {
		if (currentRecord) {
			let line = "";
			for (let tag of currentRecord.Tags) {
				if (line.length > 0) {
					line += ", ";
				}

				line += `-${tag}`;
			}

			if (line.length > 0) {
				addTags(line, true);
			}
		}
	};

	const undoRemoves = (delta: IDelta) => {
		let line = "";
		for (let removed of delta.removed) {
			if (line.length > 0) {
				line += ", ";
			}

			line += removed;
		}

		if (line.length > 0) {
			addTags(line, true);
		}
	};

	const handleTagLine = (override?: string) => {
		setTagLine("");
		let tempTagLine = override || tagLine;
		if (tempTagLine.length > 0) {
			if (tempTagLine[0] === "<" || tempTagLine[0] === ">") {
				let direction = tempTagLine[0] === "<" ? -1 : 1;
				let amount = parseInt(tempTagLine.substring(1));
				if (isNaN(amount)) {
					amount = 1;
				}
				setRecordIndex(recordIndex + direction * amount);
			} else if (
				tempTagLine[tempTagLine.length - 1] === "<" ||
				tempTagLine[tempTagLine.length - 1] === ">"
			) {
				let direction = tempTagLine[0] === "<" ? -1 : 1;
				let amount = parseInt(
					tempTagLine.substring(0, tempTagLine.length - 1)
				);
				if (isNaN(amount)) {
					amount = 1;
				}
				setRecordIndex(recordIndex + direction * amount);
			} else {
				// TODO: is this safe?
				addTags(tempTagLine);
			}
		} else {
			setRecordIndex(recordIndex + 1);
		}
	};

	return (
		<div className="app">
			<ResizablePanel
				startingValue={400}
				position="left"
				className="sidebar"
			>
				<Tabs
					className="fancy-tabs"
					defaultActiveKey="search"
					activeKey={tabKey}
					onSelect={(k) =>
						updateTabKey((k || null) as ITabKey | null)
					}
				>
					<Tab eventKey="search" title="search">
						<div className="growable-container">
							<div className="filter-tab-container growable">
								<SpecialInputSimple
									onChange={setSearch}
									action={loadData}
									actionName="Search"
									focusEpoch={searchFocusEpoch}
									value={search}
								/>

								<DisplayFilters
									filters={filters}
									setFilters={setFilters}
								/>
							</div>

							<Drawer startingValue={350} position="bottom">
								{isLoadingSearches ? (
									<Spinner
										animation="border"
										role="status"
										className="centered"
									>
										<span className="sr-only">
											Loading...
										</span>
									</Spinner>
								) : (
									<DisplaySearches
										searches={searches}
										setSearches={setSearches}
										action={(search: string) => {
											loadData(search);
											setSearch(search);
										}}
									/>
								)}
							</Drawer>
						</div>
					</Tab>

					<Tab
						eventKey="play"
						title={
							records.length
								? `play (${recordIndex + 1} / ${
										records.length
								  })`
								: "play"
						}
					>
						{currentRecord ? (
							<div className="tag-input-container growable-container">
								<DisplayRecord record={currentRecord} />

								<div className="fancy-button-bar">
									<OverlayTrigger
										placement="top"
										overlay={SimpleTooltip(
											"Reload records"
										)}
									>
										<Button
											size="sm"
											variant="dark"
											onClick={async () => {
												await bridge.reload();
												loadData();
											}}
										>
											<ArrowClockwise />
										</Button>
									</OverlayTrigger>

									<OverlayTrigger
										placement="top"
										overlay={SimpleTooltip("Open natively")}
									>
										<Button
											onClick={() =>
												openNatively(
													currentRecord.Location
												)
											}
											size="sm"
											variant="dark"
										>
											Open
										</Button>
									</OverlayTrigger>

									<OverlayTrigger
										placement="top"
										overlay={SimpleTooltip(
											"Open containing folder"
										)}
									>
										<Button
											onClick={() =>
												openContainingFolder(
													currentRecord.Location
												)
											}
											size="sm"
											variant="dark"
										>
											Folder
										</Button>
									</OverlayTrigger>

									<OverlayTrigger
										placement="top"
										overlay={SimpleTooltip(
											"Remove all tags"
										)}
									>
										<Button
											onClick={() => clearTags()}
											size="sm"
											variant="danger"
											disabled={
												currentRecord.Tags.length === 0
											}
										>
											Clear Tags
										</Button>
									</OverlayTrigger>

									{/* <Button
										onClick={() => removeSelf()}
										size="sm"
										variant="danger"
									>
										Delete
									</Button> */}

									{/* <Button
										onClick={() => editSelf()}
										size="sm"
										variant="secondary"
									>
										Edit
									</Button> */}

									<Button
										onClick={() => handleTagLine("reset")}
										size="sm"
										variant="dark"
									>
										Reset
									</Button>

									{/* <Button
										onClick={() => reload()}
										size="sm"
										variant="dark"
									>
										Reload
									</Button> */}

									<Button
										onClick={() => previousRecord()}
										size="sm"
										variant="dark"
									>
										<ArrowLeft />
									</Button>
									<Button
										onClick={() => nextRecord()}
										size="sm"
										variant="dark"
									>
										<ArrowRight />
									</Button>
								</div>

								<SpecialInput
									focusEpoch={tagFocusEpoch}
									action={handleTagLine}
									options={allTags}
									// onChange={updateTagLine}
									// value={tagLine}
									// actionName="Add"
								/>

								{recommendedTags.length ? (
									<div className="recommended-tags">
										<h4>
											Recommended Tags{" "}
											<Button
												size="sm"
												variant="dark"
												onClick={() =>
													addAllRecommended()
												}
											>
												Add All
											</Button>
										</h4>
										<div className="tag-container">
											{recommendedTags.map((tag) => (
												<DisplayTagLineGroup
													tag={tag}
													key={tag}
													variant="secondary"
													action={(
														tagName: string
													) => {
														addTagLine(tagName);
														setRecommendedTags(
															(tags) =>
																tags.filter(
																	(tag) =>
																		tag !==
																		tagName
																)
														);
													}}
												/>
											))}
										</div>
									</div>
								) : null}

								<div className="growable">
									<div className="tag-container">
										{currentRecord.Tags.map((tag) => (
											<DisplayTagLineGroup
												tag={tag}
												key={tag}
												variant="success"
												action={(tagName: string) => {
													toast(
														`hooray for ${tagName}`
													);
													setSearch(tagName);
													loadData(tagName);
												}}
												rightClickAction={removeTagLine}
												// secondaryTitle="?"
											/>
										))}
									</div>
								</div>

								<Drawer
									startingValue={350}
									position="bottom"
									closed
								>
									<DisplayDeltas
										deltas={deltas}
										setDeltas={setDeltas}
										undoAdds={undoAdds}
										undoRemoves={undoRemoves}
										addTagLine={addTagLine}
										removeTagLine={removeTagLine}
										onDoubleClick={() => nextRecord()}
									/>
								</Drawer>
							</div>
						) : (
							<Jumbotron fluid className="fancy-jumbotron">
								<h1>No results: (</h1>
							</Jumbotron>
						)}
					</Tab>

					<Tab eventKey="edit_settings" title="settings">
						<Jumbotron fluid className="fancy-jumbotron">
							<Button onClick={() => bridge.editSettings()}>
								Edit In VSCode
							</Button>

							<Button
								onClick={async () => {
									await bridge.reload();
									loadData();
								}}
							>
								Reload
							</Button>

							<Button onClick={() => setInitialized(false)}>
								Folder Select
							</Button>
						</Jumbotron>
					</Tab>
				</Tabs>
			</ResizablePanel>

			<Content record={currentRecord} />

			<ToastContainer
				draggable={false}
				autoClose={2000}
				hideProgressBar
				limit={5}
				position="bottom-right"
			/>
		</div>
	);
}

function sortTags(newTags: string[], oldTags: string[]): string[] {
	// Keeps the order of tags that are in both oldTags and newTags,
	// but keep the tags that are just in newTags at the top
	let existing = [];
	let netNew = [];
	for (let tag of oldTags) {
		if (newTags.indexOf(tag) > -1) {
			existing.push(tag);
		}
	}

	for (let tag of newTags) {
		if (oldTags.indexOf(tag) === -1) {
			netNew.push(tag);
		}
	}

	return netNew.concat(existing);
}

function updateRecord(
	setRecords: (fn: (records: IRecord[]) => IRecord[]) => any,
	updatedRecord: IRecord
) {
	setRecords((records) =>
		records.map((record) =>
			record.RecordID === updatedRecord.RecordID ? updatedRecord : record
		)
	);
}

function difference<T>(a: T[], b: T[]) {
	let rv = [];
	for (let x of a) {
		if (b.indexOf(x) === -1) {
			rv.push(x);
		}
	}
	return rv;
}

function getSearch(
	search: string,
	filters: IFilter[],
	override?: string
): string {
	let tempSearch = override || search;
	if (tempSearch.length > 0) {
		if (
			!tempSearch.startsWith("search") &&
			!tempSearch.startsWith("-sort")
		) {
			tempSearch = `search ${tempSearch}`;
		}
	}

	for (let filter of filters) {
		// if we require a value make sure it exists
		// need to !! to get to bools
		if (filter.base.valueType) {
			if (filter.value !== null && filter.value !== undefined) {
				tempSearch += ` ${filter.base.command}`;
				tempSearch += ` ${filter.value}`;
			}
		} else {
			tempSearch += ` ${filter.base.command}`;
		}
	}

	return tempSearch;
}

async function openNatively(location: string) {
	await bridge.openNatively({ location });
}

async function openContainingFolder(location: string) {
	await bridge.openContainingFolder({ location });
}

function parseWords(words: string) {
	return (words.match(/[^\s"]+|"([^"]*)"/gi) || []).map((word) =>
		word.replace(/^"(.+(?="$))"$/, "$1")
	);
}

export default App;
