import React from "react";
import { ReactChildren, useEffect, useState } from "react";
import Draggable from "react-draggable";
import { transform } from "typescript";

type IResizablePanel = React.PropsWithChildren<{
	startingValue: number;
	position: "top" | "bottom" | "left" | "right";
	className?: string;
}>;

export default function ResizablePanel({
	startingValue,
	position,
	className,
	children,
}: IResizablePanel) {
	const [value, setValue] = useState(startingValue);
	const [isClosed, setIsClosed] = useState(false);
	const [didDrag, setDidDrag] = useState(false);
	const [deltaPosition, setDeltaPosition] = useState({
		x: 0,
		y: 0,
	});

	className = className || "";

	useEffect(() => {
		if (position === "left" || position === "right") {
			setValue(startingValue + deltaPosition.x);
		} else {
			setValue(startingValue - deltaPosition.y);
		}
	}, [deltaPosition, startingValue]);

	const onStart = () => {
		setDidDrag(false);
	};

	const handleDrag = (e: any, ui: any) => {
		const { x, y } = deltaPosition;
		setDeltaPosition({
			x: x + ui.deltaX,
			y: y + ui.deltaY,
		});
		setDidDrag(true);
	};

	const onStop = () => {
		// click, essentially
		console.log(didDrag);
		if (!didDrag) {
			setIsClosed(true);
		}
	};

	let tempValue = value;
	if (isClosed) {
		tempValue = 0;
	}

	let primarySize = 5;
	let secondarySize = 100;
	let handleMargin = startingValue - primarySize / 2;

	let containerStyle: any = {};
	let handleStyle: any = {}; // react doesn't like position absolute for some reason
	let axis = "x";
	switch (position) {
		case "left":
			containerStyle = { width: tempValue, minWidth: tempValue };
			handleStyle = {
				width: primarySize,
				// height: secondarySize,
				height: "100%",
				marginLeft: handleMargin,
				marginTop: "auto",
				marginBottom: "auto",
				top: 0,
				bottom: 0,
				position: "absolute",
			};
			break;

		case "top":
			containerStyle = { height: tempValue, minHeight: tempValue };
			handleStyle = {
				width: secondarySize,
				height: primarySize,
				marginTop: handleMargin,
				marginLeft: "auto",
				marginRight: "auto",
				left: 0,
				right: 0,
				position: "absolute",
			};
			axis = "y";
			break;

		case "bottom":
			containerStyle = { height: tempValue, minHeight: tempValue };
			handleStyle = {
				// width: secondarySize,
				height: primarySize,
				// marginBottom: handleMargin,
				// marginTop: -(handleMargin + deltaPosition.y),
				marginLeft: "auto",
				marginRight: "auto",
				left: 0,
				right: 0,
				bottom: 0,
				position: "absolute",
			};
			axis = "y";
			break;
	}

	if (isClosed) {
		handleStyle.left = 0;
		handleStyle.transform = null;
		switch (position) {
			case "left":
				{
					let margin = -deltaPosition.x;
					handleStyle.marginLeft = margin;
				}
				break;

			case "top":
				{
					let margin = -deltaPosition.y;
					handleStyle.marginTop = margin;
				}
				break;

			case "bottom":
				{
					let margin = -deltaPosition.y;
					handleStyle.marginBottom = margin;
				}
				break;
		}
	}

	return (
		<div
			className={`${isClosed ? "closed" : ""} ${className}`}
			style={{ ...containerStyle }}
		>
			{children}

			<Draggable
				axis={axis as any}
				onStart={onStart}
				onDrag={handleDrag}
				onStop={onStop}
				onMouseDown={() => {
					if (isClosed) {
						setIsClosed(false);
					}
				}}
				disabled={isClosed}
			>
				<div
					style={{ ...handleStyle }}
					className="resizable-panel-handle"
				/>
			</Draggable>
		</div>
	);
}
