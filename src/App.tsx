import React, { useState } from "react";
import clamp from "lodash-es/clamp";
import { useSpring, animated } from "react-spring";
import { useGesture } from "react-with-gesture";

import "./App.css";
import { bridge } from "./Commands";

function App() {
	const [tags, setTags] = useState([] as String[]);
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
		let newTags = await bridge.get_tags();
		setTags(newTags);
	};

	const _onClick = (_: any) => {
		setClick(!clicked);
		doit();
	};

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
							{tags.map((tag, i) => (
								<span key={i}>sup{tag}</span>
							))}
						</span>
					</animated.div>
				</div>
				<div className="container"></div>
			</header>
		</div>
	);
}

export default App;
