import React, { useEffect, useState } from "react";

import "./App.css";
import { bridge } from "./Commands";
import { IRecord } from "./interfaces";
import { ChangeEvent } from "react";
import { Content } from "./Content";

function App() {
	const [search, setSearch] = useState("");
	const [tagLine, setTagLine] = useState("");
	const [records, setRecords] = useState([] as IRecord[]);

	const [recordIndex, setRecordIndex] = useState(0);
	const [currentRecord, setCurrentRecord] = useState(null as IRecord | null);

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
		setRecordIndex(0);
	};

	const addTags = async () => {
		if (!currentRecord) {
			return;
		}

		let newRecord = await bridge.add_tags({
			record: currentRecord,
			tag_line: tagLine,
		});

		// TODO: better tag sorting
		newRecord.Tags.sort();

		setRecords(
			records.map((record) =>
				record.RecordID === newRecord.RecordID ? newRecord : record
			)
		);
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
				addTags();
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
						/>

						<ul>
							{currentRecord.Tags.map((tag) => (
								<li key={tag}>{tag}</li>
							))}
						</ul>

						{/* <button>Open Natively</button> */}
					</div>
				) : null}
			</div>

			<div className="content">{<Content record={currentRecord} />}</div>
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
}
function SpecialInput({
	className,
	key,
	action,
	onChange,
	value,
	actionName,
	prefix,
}: ISpecialInput) {
	return (
		<div className="special-input">
			{prefix ? <span onClick={() => action()}>{prefix}</span> : null}

			<input
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

function printSize(bytes: number) {
	return `${bytes}b`;
}

export default App;
