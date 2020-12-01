import React, { useEffect, useState } from "react";
import clamp from "lodash-es/clamp";
import { useSpring, animated } from "react-spring";
import { useGesture } from "react-with-gesture";

import "./App.css";
import { bridge } from "./Commands";
import { IRecord } from "./interfaces";
import { setUncaughtExceptionCaptureCallback } from "process";
import { ChangeEvent } from "react";
import { sortedLastIndex } from "lodash-es";

function App() {
	const [search, setSearch] = useState("");
	const [tagLine, setTagLine] = useState("");
	const [records, setRecords] = useState([] as IRecord[]);

	const [recordIndex, setRecordIndex] = useState(0);
	const [currentRecord, setCurrentRecord] = useState(null as null | IRecord);

	const loadData = async () => {
		let records = await bridge.get_records({ args: search.split(" ") });
		setRecords(records);
		setRecordIndex(0);
	};

	useEffect(() => {
		if (records.length == 0) {
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
	}, [recordIndex, records]);

	const addTags = async () => {
		if (!currentRecord) {
			return;
		}

		let newRecord = await bridge.add_tags({
			record: currentRecord,
			tag_line: tagLine,
		});

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
		<div className="App">
			<div className="sidebar">
				<input onChange={updateSearch} value={search} />
				<button onClick={loadData}>Refresh</button>
				<div>
					<h1>Names</h1>
					<ul>
						{records.map((record) => (
							<>
								<li
									key={record.RecordID}
									className={
										record === currentRecord
											? "special"
											: ""
									}
								>
									{record.Name}
								</li>

								<ul>
									{record.Tags.map((tag) => (
										<li key={tag}>{tag}</li>
									))}
								</ul>
							</>
						))}
					</ul>
				</div>

				{currentRecord ? (
					<input
						onChange={updateTagLine}
						onKeyPress={(event) => {
							if ((event.keyCode || event.which) === 13) {
								handleTagLine();
							}
						}}
						value={tagLine}
					/>
				) : null}
			</div>

			<div className="content"></div>
			{/* <header className="App-header">
				<div className="overlay" />
				<div className="sticker">
					<animated.div
						{...bind()}
						style={{
							transform: xy.interpolate(
								((x: any, y: any) =>
									`translate3d(${x}px,${y}px,0)`) as any
							),
						}}
					>
						<span onClick={_onClick}>
							{!clicked ? "gtaggenator" : "Sup!"}
							{tags.map((tag, i) => (
								<span key={i}>sup{tag}</span>
							))}
						</span>
					</animated.div>
				</div>
				<div className="container"></div>
			</header> */}
		</div>
	);
}

export default App;
