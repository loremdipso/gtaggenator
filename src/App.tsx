import React, { useEffect, useState } from "react";
import clamp from "lodash-es/clamp";
import { useSpring, animated } from "react-spring";
import { useGesture } from "react-with-gesture";

import "./App.css";
import { bridge } from "./Commands";
import { IRecord } from "./interfaces";

function App() {
	const [records, setRecords] = useState([] as IRecord[]);
	const [clicked, setClick] = useState(false);
	const [{ xy }, set] = useSpring(() => ({ xy: [0, 0] }));
	const bind = useGesture(({ down, delta, velocity }) => {
		velocity = clamp(velocity, 1, 8);
		set({
			xy: down ? delta : [0, 0],
			config: { mass: velocity, tension: 500 * velocity, friction: 50 },
		});
	});

	const doit = async () => {
		// await bridge.add_tags({ recordId: 1, tags: ["search", "yup"] });
		// let newTags = await bridge.get_tags({ args: ["search", "yup"] });
		let records = await bridge.get_records({ args: [] });
		setRecords(records);
	};

	useEffect(() => {
		doit();
	}, []);

	const _onClick = (_: any) => {
		setClick(!clicked);
		doit();
	};

	return (
		<div className="App">
			{records.map((record) => (
				<span key={record.RecordID}>{record.Name}</span>
			))}
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
