import React, { useCallback, useEffect, useRef, useState } from "react";

import "./App.css";
import { bridge } from "./Utils/Commands";
import { IRecord } from "./Utils/interfaces";
import { ChangeEvent } from "react";
import { Content } from "./Components/Content";
import {
	IDelta,
	createDelta,
	DisplayDeltas,
	DisplayTagLineGroup,
	appendDeltaImmutable,
} from "./Components/Deltas";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

import { Button, OverlayTrigger, Tab, Tabs, Tooltip } from "react-bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";

import DisplayRecord from "./Components/DisplayRecord";
import { useHotkeys } from "react-hotkeys-hook";

import ResizablePanel from "./Components/ResizablePanel";
import { useHotkeysHelper } from "./Utils/Hotkeys";
import { DisplayFilters, IFilter } from "./Components/Filters";
import Drawer from "./Components/Drawer";
import { RecoilRoot, useRecoilState } from "recoil";
import { currentRecordIndex, fileServerPort } from "./Utils/Atoms";

type ITabKey = "search" | "play";

function App() {
	return (
		<RecoilRoot>
			<AppContent />
		</RecoilRoot>
	);
}

function AppContent() {
	const [args, setArgs] = useState([] as string[]);

	const [search, setSearch] = useState("");
	const [tagLine, setTagLine] = useState("");
	const [records, setRecords] = useState([] as IRecord[]);

	const [filters, setFilters] = useState([] as IFilter[]);
	const [deltas, setDeltas] = useState([] as IDelta[]);

	const [lastExecutedSearch, setLastExecutedSearch] = useState("");

	const [tagFocusEpoch, setTagFocusEpoch] = useState(0);
	const [searchFocusEpoch, setSearchFocusEpoch] = useState(0);

	const [recordIndex, setRecordIndex] = useRecoilState(currentRecordIndex);
	const [currentRecord, setCurrentRecord] = useState(null as IRecord | null);

	const [tabKey, setTabKey] = useState("search" as ITabKey);

	const [lastOpenedRecordID, setLastOpenedRecordID] = useState(
		null as number | null
	);

	const [_, setPort] = useRecoilState(fileServerPort);
	useEffect(() => {
		(async () => {
			let port = await bridge.getPort();
			console.log(`Using port ${port}`);
			setPort(port);
		})();
	}, []);

	const updateTabKey = (key: ITabKey | null) => {
		if (!key || tabKey === key) {
			return;
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

	// load initial arguments, once
	useEffect(() => {
		(async () => {
			let initialArgs = await bridge.getInitialArguments();
			setArgs(initialArgs);
		})();
	}, []);

	useEffect(() => {
		if (args.length) {
			console.log(`initial args: ${args}`);
			let argsString = args.join(" ");
			setSearch(argsString);
			loadData(argsString);
			// TODO: populate filters from this
		}
	}, [args]);

	const updateRecord = useCallback(
		(updatedRecord: IRecord) => {
			setRecords(
				records.map((record) =>
					record.RecordID === updatedRecord.RecordID
						? updatedRecord
						: record
				)
			);
		},
		[records, setRecords]
	);

	useEffect(() => {
		(async () => {
			if (
				currentRecord &&
				currentRecord.RecordID !== lastOpenedRecordID
			) {
				// we've got to keep track of this, otherwise we get an infinite loop, which is no bueno
				setLastOpenedRecordID(currentRecord.RecordID);
				// UI minor change: we'll update the record immediately, even before the request is finished
				updateRecord({
					...currentRecord,
					TimesOpened: currentRecord.TimesOpened + 1,
				});
				await bridge.open_record({
					record: currentRecord,
				});

				// updateRecord(
				// 	await bridge.open_record({
				// 		record: currentRecord,
				// 	})
				// );
			}
		})();
	}, [currentRecord, lastOpenedRecordID, updateRecord]);

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
	}, [recordIndex, records, currentRecord]);

	const doEnd = () => {
		setRecords([]);
		setTabKey("search");
		setSearchFocusEpoch((oldEpoch) => oldEpoch + 1);
	};

	const getSearch = (override?: string) => {
		let tempSearch = override || search;
		if (tempSearch.length > 0) {
			if (!tempSearch.startsWith("search")) {
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
	};

	const loadData = async (override?: string) => {
		let tempSearch = getSearch(override);

		try {
			let records = await bridge.get_records({
				args: tempSearch.split(" "),
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
			setTagFocusEpoch(tagFocusEpoch + 1);
			setLastExecutedSearch(tempSearch);
		} catch (e) {
			console.log(e);
		}
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
		let newRecord = await bridge.add_tags({
			record: currentRecord,
			tag_line: line,
		});

		if (!ignoreDelta) {
			let delta = createDelta(newRecord.Tags, oldRecord.Tags, line);
			if (delta.added.length || delta.removed.length) {
				// insert after all our favorite deltas
				setDeltas((deltas) => appendDeltaImmutable(deltas, delta));
			}
		}

		newRecord.Tags = sortTags(newRecord.Tags, oldRecord.Tags);
		updateRecord(newRecord);
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

	const reload = () => {
		loadData();
	};

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

	const updateTagLine = (event: ChangeEvent<HTMLInputElement>) => {
		setTagLine(event.target.value);
	};

	const updateSearch = (event: ChangeEvent<HTMLInputElement>) => {
		setSearch(event.target.value);
	};

	const handleTagLine = () => {
		setTagLine("");
		if (tagLine.length > 0) {
			if (tagLine[0] === "<" || tagLine[0] === ">") {
				let direction = tagLine[0] === "<" ? -1 : 1;
				let amount = parseInt(tagLine.substring(1));
				if (isNaN(amount)) {
					amount = 1;
				}
				setRecordIndex(recordIndex + direction * amount);
			} else if (
				tagLine[tagLine.length - 1] === "<" ||
				tagLine[tagLine.length - 1] === ">"
			) {
				let direction = tagLine[0] === "<" ? -1 : 1;
				let amount = parseInt(tagLine.substring(0, tagLine.length - 1));
				if (isNaN(amount)) {
					amount = 1;
				}
				setRecordIndex(recordIndex + direction * amount);
			} else {
				// TODO: is this safe?
				addTags(tagLine);
			}
		} else {
			setRecordIndex(recordIndex + 1);
		}
	};

	return (
		<div className="app">
			<ResizablePanel
				startingValue={350}
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
						<div className="filter-tab-container">
							<SpecialInput
								onChange={updateSearch}
								action={loadData}
								actionName="Search"
								value={search}
								focusEpoch={searchFocusEpoch}
							/>

							<DisplayFilters
								filters={filters}
								setFilters={setFilters}
							/>
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
							<div className="tag-input-container">
								<DisplayRecord record={currentRecord} />

								<SpecialInput
									onChange={updateTagLine}
									action={handleTagLine}
									value={tagLine}
									actionName="Add"
									focusEpoch={tagFocusEpoch}
								/>

								<div className="fancy-button-bar">
									{/* <OverlayTrigger
											placement="top"
											overlay={myTooltip(
												"Try to launch on local machine"
											)}
										>
											<Button
												onClick={() => reload()}
												size="sm"
												variant="dark"
											>
												Launch
											</Button>
										</OverlayTrigger> */}

									<OverlayTrigger
										placement="top"
										overlay={myTooltip("Remove all tags")}
									>
										<Button
											onClick={() => clearTags()}
											size="sm"
											variant="dark"
											disabled={
												currentRecord.Tags.length === 0
											}
										>
											Clear
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
										onClick={() => reload()}
										size="sm"
										variant="dark"
									>
										Reload
									</Button>

									<Button
										onClick={() => previousRecord()}
										size="sm"
										variant="dark"
									>
										{"<"}
									</Button>
									<Button
										onClick={() => nextRecord()}
										size="sm"
										variant="dark"
									>
										{">"}
									</Button>
								</div>

								<div className="growable">
									<div className="tag-container">
										{sortRecordTags(currentRecord).Tags.map(
											(tag) => (
												<DisplayTagLineGroup
													tag={tag}
													key={tag}
													variant="success"
													action={(
														tagName: string
													) => {
														toast(
															`hooray for ${tagName}`
														);
														setSearch(tagName);
														loadData(tagName);
													}}
													rightClickAction={
														removeTagLine
													}
													// secondaryTitle="?"
												/>
											)
										)}
									</div>
								</div>

								<Drawer startingValue={350} position="bottom">
									<DisplayDeltas
										deltas={deltas}
										setDeltas={setDeltas}
										undoAdds={undoAdds}
										undoRemoves={undoRemoves}
										addTagLine={addTagLine}
										removeTagLine={removeTagLine}
									/>
								</Drawer>
							</div>
						) : (
							<div>No results: (</div>
						)}
					</Tab>
				</Tabs>
			</ResizablePanel>

			<Content record={currentRecord} />

			<ToastContainer
				draggable={false}
				autoClose={2000}
				hideProgressBar
				position="bottom-right"
			/>
		</div>
	);
}

interface ISpecialInput {
	className?: string;
	key?: string;
	action: Function;
	onChange: (event: ChangeEvent<HTMLInputElement>) => void;
	value: string;
	actionName?: string;
	prefix?: string;
	extra?: React.ReactElement;

	focusEpoch?: number;
}
function SpecialInput({
	className,
	action,
	onChange,
	value,
	actionName,
	prefix,
	extra,
	focusEpoch, // used to force focus
}: ISpecialInput) {
	const thisInput = useRef<HTMLInputElement>(null);
	useEffect(() => {
		if (focusEpoch && thisInput.current) {
			thisInput.current.focus();
			thisInput.current.select();
		}
	}, [thisInput, focusEpoch]);

	return (
		<div className="special-input">
			{prefix ? <button onClick={() => action()}>{prefix}</button> : null}

			<input
				ref={thisInput}
				value={value}
				className={className}
				onChange={onChange}
				onKeyPress={(event) => {
					if ((event.keyCode || event.which) === 13) {
						action();
					}
				}}
			/>

			{actionName ? (
				<button onClick={() => action()}>{actionName}</button>
			) : null}

			{extra}
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

function sortRecordTags(record: IRecord): IRecord {
	if (!record.OpenedInGUI) {
		record.Tags.sort();
		record.OpenedInGUI = true;
	}
	return record;
}

export default App;

function myTooltip(text: string) {
	return (props: any) => <Tooltip {...props}>{text}</Tooltip>;
}
