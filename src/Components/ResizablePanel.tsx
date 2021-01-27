import React from "react";
import { useEffect, useState } from "react";
import Draggable from "react-draggable";

type IResizablePanel = React.PropsWithChildren<{
	startingValue: number;
	position: "top" | "bottom" | "left" | "right";
	className?: string;
	fullSize?: boolean;
}>;

export default function ResizablePanel({
	startingValue,
	position,
	className,
	children,
	fullSize,
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
		if (position === "left") {
			setValue(startingValue + deltaPosition.x);
		} else if (position === "right") {
			setValue(startingValue - deltaPosition.x);
		} else {
			setValue(startingValue - deltaPosition.y);
		}
	}, [position, deltaPosition, startingValue]);

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
				height: "100%",
				marginLeft: handleMargin,
				marginTop: "auto",
				marginBottom: "auto",
				top: 0,
				bottom: 0,
				position: "absolute",
			};
			break;

		case "right":
			containerStyle = { width: tempValue, minWidth: tempValue };
			handleStyle = {
				width: primarySize,
				height: "100%",
				marginRight: handleMargin,
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
				height: primarySize,
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
		handleStyle.transform = null;
		switch (position) {
			case "left":
				{
					let margin = -deltaPosition.x;
					handleStyle.left = 0;
					handleStyle.marginLeft = margin;
				}
				break;

			case "right":
				{
					let margin = deltaPosition.x;
					handleStyle.marginRight = margin;
					handleStyle.right = 0;
				}
				break;

			case "top":
				{
					// TODO: test
					let margin = -deltaPosition.y;
					handleStyle.left = 0;
					handleStyle.marginTop = margin;
				}
				break;

			case "bottom":
				{
					let margin = -deltaPosition.y;
					handleStyle.left = 0;
					handleStyle.marginBottom = margin;
				}
				break;
		}
	}

	if (fullSize) {
		return <div className={`${className} full-size`}>{children}</div>;
	}

	return (
		<div
			className={`${isClosed ? "closed" : ""} ${className}`}
			style={{ ...containerStyle }}
		>
			{isClosed ? null : children}

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
