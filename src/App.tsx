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

	const loadData = async () => {
		let records = await bridge.get_records({ args: search.split(" ") });
		setRecords(records);
		setRecordIndex(0);
	};

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
				<div className="fw">
					<input onChange={updateSearch} value={search} />
					<button onClick={loadData}>Refresh</button>
				</div>

				{currentRecord ? (
					<div>
						<div>
							{recordIndex + 1} / {records.length}
						</div>

						<ul>
							{currentRecord.Tags.map((tag) => (
								<li key={tag}>{tag}</li>
							))}
						</ul>

						<input
							className="fw"
							onChange={updateTagLine}
							onKeyPress={(event) => {
								if ((event.keyCode || event.which) === 13) {
									handleTagLine();
								}
							}}
							value={tagLine}
						/>

						<button>Open Natively</button>
					</div>
				) : null}
			</div>

			<div className="content">{<Content record={currentRecord} />}</div>
		</div>
	);
}

export default App;
