import React, { useEffect, useState } from "react";
import clamp from "lodash-es/clamp";
import { useSpring, animated } from "react-spring";
import { useGesture } from "react-with-gesture";
import { promisified } from "tauri/api/tauri";

import "./App.css";

function App() {
	const [clicked, setClick] = useState(false);
	const [{ xy }, set] = useSpring(() => ({ xy: [0, 0] }));
	const bind = useGesture(({ down, delta, velocity }) => {
		velocity = clamp(velocity, 1, 8);
		set({
			xy: down ? delta : [0, 0],
			config: { mass: velocity, tension: 500 * velocity, friction: 50 },
		});
	});

	useEffect(() => {
		(async () => {
			console.log("A");
			let thing = await promisified({
				cmd: "doSomethingAsync",
				count: 6,
				payload: {
					state: "some string data",
					data: 17,
				},
			});
			console.log("B");
			console.log(thing);
		})();
	});

	const _onClick = (_: any) => setClick(!clicked);

	return (
		<div className="App">
			<header className="App-header">
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
						</span>
					</animated.div>
				</div>
				<div className="container"></div>
			</header>
		</div>
	);
}

export default App;
