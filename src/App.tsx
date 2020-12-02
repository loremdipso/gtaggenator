import React, { useCallback, useEffect, useRef, useState } from "react";

import "./App.css";
import { bridge } from "./Commands";
import { IRecord } from "./interfaces";
import { ChangeEvent } from "react";
import { Content } from "./Content";
import {
	IDelta,
	createDelta,
	DisplayDeltas,
	DisplayTagLineGroup,
	appendDeltaImmutable,
} from "./Deltas";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

import { Dropdown } from "react-bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";

import DisplayRecord from "./DisplayRecord";
import { useHotkeys } from "react-hotkeys-hook";

import ResizablePanel from "./ResizablePanel";
import { useHotkeysHelper } from "./Utils";

interface IFilter {
	display: string;
	command: string;
}
const filters: IFilter[] = [
	{ display: "None", command: "" },
	{ display: "Untouched", command: "-sort untouched" },
	{ display: "Touched", command: "-sort touched" },
	{ display: "Seen", command: "-sort seen" },
	{ display: "Unseen", command: "-sort unseen" },
	{ display: "Most Tags", command: "-sort most_tags" },
	{ display: "Fewest Tags", command: "-sort fewest_tags" },
];

function App() {
	const [search, setSearch] = useState("");
	const [tagLine, setTagLine] = useState("");
	const [records, setRecords] = useState([] as IRecord[]);

	const [lastExecutedSearch, setLastExecutedSearch] = useState("");

	const [tagFocusEpoch, setTagFocusEpoch] = useState(0);
	const [searchFocusEpoch, setSearchFocusEpoch] = useState(0);

	const [recordIndex, setRecordIndex] = useState(0);
	const [currentRecord, setCurrentRecord] = useState(null as IRecord | null);
	const [deltas, setDeltas] = useState([] as IDelta[]);

	const [currentFilter, setCurrentFilter] = useState(filters[0]);
	const [lastOpenedRecordID, setLastOpenedRecordID] = useState(
		null as number | null
	);

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
		[nextRecord, previousRecord]
	);

	useHotkeysHelper(
		"alt+numpad/",
		() => {
			previousRecord();
		},
		[nextRecord, previousRecord]
	);

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
				updateRecord(
					await bridge.open_record({
						record: currentRecord,
					})
				);
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
		setSearchFocusEpoch((oldEpoch) => oldEpoch + 1);
	};

	const getSearch = () => {
		let tempSearch = search;
		if (search.length > 0) {
			if (!search.startsWith("search")) {
				tempSearch = `search ${search}`;
			}
		}

		tempSearch += ` ${currentFilter.command}`;
		return tempSearch;
	};

	const loadData = async () => {
		let tempSearch = getSearch();
		let records = await bridge.get_records({ args: tempSearch.split(" ") });
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
		setRecordIndex(newIndex);
		setTagFocusEpoch(tagFocusEpoch + 1);
		setLastExecutedSearch(tempSearch);
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

		// for (let removed of delta.removed) {
		// 	if (line.length > 0) {
		// 		line += ", ";
		// 	}
		// 	line += removed;
		// }

		if (line.length > 0) {
			// remove this delta
			// let temp = delta.added;
			// delta.added = delta.removed;
			// delta.removed = temp;
			setDeltas(deltas); // TODO: something less hacky. We reallllly shouldn't be modifying data like this
			addTags(line, true);
		}
	};

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
			setDeltas(deltas); // TODO: something less hacky. We reallllly shouldn't be modifying data like this
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
			<ResizablePanel startingValue={350} axis="x" className="sidebar">
				<SpecialInput
					onChange={updateSearch}
					action={loadData}
					value={search}
					prefix="Search"
					focusEpoch={searchFocusEpoch}
				/>

				<Dropdown>
					<Dropdown.Toggle variant="primary" id="dropdown-basic">
						Filter
					</Dropdown.Toggle>

					<Dropdown.Menu>
						{filters.map((filter) => (
							<Dropdown.Item
								key={filter.display}
								onClick={() => setCurrentFilter(filter)}
							>
								{filter.display}
							</Dropdown.Item>
						))}
					</Dropdown.Menu>
				</Dropdown>

				{currentRecord ? (
					<>
						<div className="tag-input-container">
							<DisplayRecord
								record={currentRecord}
								recordIndex={recordIndex}
								numRecords={records.length}
							/>

							<SpecialInput
								onChange={updateTagLine}
								action={handleTagLine}
								value={tagLine}
								actionName="Add"
								focusEpoch={tagFocusEpoch}
								extra={
									<button onClick={() => clearTags()}>
										Clear
									</button>
								}
							/>

							<div className="tag-container-container">
								<div className="tag-container">
									{sortRecordTags(currentRecord).Tags.map(
										(tag) => (
											<DisplayTagLineGroup
												tag={tag}
												key={tag}
												variant="success"
												action={removeTagLine}
												secondaryAction={(
													tagName: string
												) =>
													toast(
														`hooray for ${tagName}`
													)
												}
												secondaryTitle="?"
											/>
										)
									)}
								</div>
							</div>
						</div>

						<DisplayDeltas
							deltas={deltas}
							setDeltas={setDeltas}
							undoAdds={undoAdds}
							undoRemoves={undoRemoves}
							// redo={redo}
							addTagLine={addTagLine}
							removeTagLine={removeTagLine}
						/>

						{/* TODO: this */}
						{/* <button>Open Natively</button> */}
					</>
				) : null}
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
