import React, { useEffect, useState } from "react";
import clamp from "lodash-es/clamp";
import { useSpring, animated } from "react-spring";
import { useGesture } from "react-with-gesture";

import "./App.css";
import { bridge } from "./Commands";
import { IRecord } from "./interfaces";
import { setUncaughtExceptionCaptureCallback } from "process";
import { ChangeEvent } from "react";

function App() {
	const [search, setSearch] = useState("");
	const [records, setRecords] = useState([] as IRecord[]);

	const loadData = async () => {
		// let newTags = await bridge.get_tags({ args: ["search", "yup"] });
		let records = await bridge.get_records({ args: [] });
		setRecords(records);
	};

	const addTags = async () => {
		let newRecord = await bridge.add_tags({
			record: records[0],
			tag_line: search,
		});

		setRecords(
			records.map((record) =>
				record.RecordID === newRecord.RecordID ? newRecord : record
			)
		);
	};

	const updateSearch = (event: ChangeEvent<HTMLInputElement>) => {
		setSearch(event.target.value);
	};

	return (
		<div className="App">
			<button onClick={loadData}>Refresh</button>
			<div>
				<h1>Names</h1>
				<ul>
					{records.map((record) => (
						<>
							<li key={record.RecordID}>{record.Name}</li>

							<ul>
								{record.Tags.map((tag) => (
									<li key={tag}>{tag}</li>
								))}
							</ul>
						</>
					))}
				</ul>
			</div>

			<input onChange={updateSearch} value={search} />
			<button onClick={() => addTags()}>Add Tags</button>
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
