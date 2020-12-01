import React, { useEffect, useRef, useState } from "react";

import "./App.css";
import { bridge } from "./Commands";
import { IRecord } from "./interfaces";
import { ChangeEvent } from "react";
import { Content } from "./Content";
import { IDelta, getDelta, DisplayDeltas } from "./Deltas";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

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
			setRecords([]);
			setSearchFocusEpoch(searchFocusEpoch + 1);
		} else if (records.length > recordIndex) {
			setCurrentRecord(records[recordIndex]);
		}
	}, [recordIndex, records, currentRecord]);

	const loadData = async () => {
		let tempSearch = search;
		if (!search.startsWith("search")) {
			tempSearch = `search ${search}`;
		}
		let records = await bridge.get_records({ args: tempSearch.split(" ") });
		setRecords(records);

		let newIndex = 0;
		if (lastExecutedSearch === search && currentRecord) {
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
		setLastExecutedSearch(search);
	};

	const addTag = (tag: string) => {
		addTags(tag, true);
	};

	const removeTag = (tag: string) => {
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
			let delta = getDelta(newRecord.Tags, oldRecord.Tags);
			if (delta.added.length || delta.removed.length) {
				setDeltas([delta, ...deltas]);
			}
		}

		newRecord.Tags = sortTags(newRecord.Tags, oldRecord.Tags);

		setRecords(
			records.map((record) =>
				record.RecordID === newRecord.RecordID ? newRecord : record
			)
		);
	};

	// will undo a delta
	const undo = (delta: IDelta) => {
		let line = "";
		for (let added of delta.added) {
			if (line.length > 0) {
				line += ", ";
			}
			line += `-${added}`;
		}

		for (let removed of delta.removed) {
			if (line.length > 0) {
				line += ", ";
			}
			line += removed;
		}

		if (line.length > 0) {
			// remove this delta
			let temp = delta.added;
			delta.added = delta.removed;
			delta.removed = temp;
			setDeltas(deltas); // TODO: something less hacky. We reallllly shouldn't be modifying data like this
			addTags(line, true);
		}
	};

	const redo = (delta: IDelta) => {
		let line = "";
		for (let added of delta.added) {
			if (line.length > 0) {
				line += ", ";
			}
			line += added;
		}

		for (let removed of delta.removed) {
			if (line.length > 0) {
				line += ", ";
			}
			line += `-${removed}`;
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
			<div className="sidebar">
				<SpecialInput
					onChange={updateSearch}
					action={loadData}
					value={search}
					prefix="Search"
					focusEpoch={searchFocusEpoch}
				/>

				{currentRecord ? (
					<div>
						<div>
							<div>
								{recordIndex + 1} / {records.length}
							</div>
							<div>{currentRecord.Name}</div>
							<div>{printSize(currentRecord.Size)}</div>
						</div>

						<SpecialInput
							onChange={updateTagLine}
							action={handleTagLine}
							value={tagLine}
							actionName="Add"
							focusEpoch={tagFocusEpoch}
						/>

						<ul>
							{fixRecord(currentRecord).Tags.map((tag) => (
								<li key={tag}>{tag}</li>
							))}
						</ul>

						<DisplayDeltas
							deltas={deltas}
							undo={undo}
							redo={redo}
							addTag={addTag}
							removeTag={removeTag}
						/>

						{/* <button>Open Natively</button> */}
					</div>
				) : null}
			</div>

			<div className="content">{<Content record={currentRecord} />}</div>

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

	focusEpoch?: number;
}
function SpecialInput({
	className,
	key,
	action,
	onChange,
	value,
	actionName,
	prefix,
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
			{prefix ? <span onClick={() => action()}>{prefix}</span> : null}

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

			{actionName ? <a onClick={() => action()}>{actionName}</a> : null}
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

function printSize(bytes: number) {
	return `${bytes}b`;
}

function fixRecord(record: IRecord): IRecord {
	if (!record.OpenedInGUI) {
		record.Tags.sort();
		record.OpenedInGUI = true;
	}
	return record;
}

export default App;
