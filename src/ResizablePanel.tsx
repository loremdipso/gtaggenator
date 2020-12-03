import React from "react";
import { ReactChildren, useEffect, useState } from "react";
import Draggable from "react-draggable";
import { transform } from "typescript";

type IResizablePanel = React.PropsWithChildren<{
	startingValue: number;
	axis: "x" | "y";
	className?: string;
}>;

export default function ResizablePanel({
	startingValue,
	axis,
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

	useEffect(() => {
		setValue(startingValue + deltaPosition.x);
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
	let containerStyle =
		axis === "x"
			? { width: tempValue, minWidth: tempValue }
			: { height: tempValue, minHeight: tempValue };

	let primarySize = 10;
	let secondarySize = 50;
	let handleMargin = startingValue - primarySize / 2;

	let handleStyle: any = // react doesn't like position absolute for some reason
		axis === "x"
			? {
					width: primarySize,
					height: secondarySize,
					marginLeft: handleMargin,
					marginTop: "auto",
					marginBottom: "auto",
					top: 0,
					bottom: 0,
					position: "absolute",
			  }
			: {
					width: secondarySize,
					height: primarySize,
					marginTop: handleMargin,
					marginLeft: "auto",
					marginRight: "auto",
					left: 0,
					right: 0,
					position: "absolute",
			  };

	if (isClosed) {
		handleStyle.left = 0;
		handleStyle.transform = null;
		if (axis === "x") {
			let margin = -deltaPosition.x;
			handleStyle.marginLeft = margin;
		} else {
			// handleStyle.marginTop = margin;
		}
	}

	return (
		<div className={className} style={{ ...containerStyle }}>
			{children}

			<Draggable
				axis={axis}
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
