import React from "react";
import { ReactChildren, useEffect, useState } from "react";
import Draggable from "react-draggable";

type IResizablePanel = React.PropsWithChildren<{
	startingValue: number;
	axis: "x" | "y";
	className?: string;
}>;

// NOTE: keep in sync with JS
const HANDLE_WIDTH = 20;

export default function ResizablePanel({
	startingValue,
	axis,
	className,
	children,
}: IResizablePanel) {
	const [value, setValue] = useState(startingValue);
	const [deltaPosition, setDeltaPosition] = useState({
		x: 0,
		y: 0,
	});

	useEffect(() => {
		setValue(startingValue + deltaPosition.x);
	}, [deltaPosition, startingValue]);

	const handleDrag = (e: any, ui: any) => {
		const { x, y } = deltaPosition;
		setDeltaPosition({
			x: x + ui.deltaX,
			y: y + ui.deltaY,
		});
	};

	const onStart = () => {};
	const onStop = () => {};

	let containerStyle =
		axis === "x"
			? { width: value, minWidth: value }
			: { height: value, minHeight: value };

	let handleMargin = startingValue - HANDLE_WIDTH / 2;
	let handleStyle: any = // react doesn't like position absolute for some reason
		axis === "x"
			? {
					width: 20,
					height: 50,
					marginLeft: handleMargin,
					backgroundColor: "black",
					marginTop: "auto",
					marginBottom: "auto",
					top: 0,
					bottom: 0,
					position: "absolute",
			  }
			: {
					width: 50,
					height: 20,
					marginTop: handleMargin,
					backgroundColor: "black",
					marginLeft: "auto",
					marginRight: "auto",
					left: 0,
					right: 0,
					position: "absolute",
			  };

	return (
		<div className={className} style={{ ...containerStyle }}>
			{children}

			<Draggable
				axis={axis}
				onStart={() => {}}
				onDrag={handleDrag}
				onStop={() => {}}
			>
				<div style={{ ...handleStyle }} />
			</Draggable>
		</div>
	);
}
