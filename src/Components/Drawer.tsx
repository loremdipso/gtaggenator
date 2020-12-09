import { RSA_PSS_SALTLEN_AUTO } from "constants";
import React from "react";
import { ReactChildren, useEffect, useState } from "react";
import Draggable from "react-draggable";
import { transform } from "typescript";

type IResizablePanel = React.PropsWithChildren<{
	startingValue: number;
	position: "top" | "bottom" | "left" | "right";
	className?: string;
	closed?: boolean;
}>;

export default function Drawer({
	startingValue,
	position,
	className,
	children,
	closed,
}: IResizablePanel) {
	const [isClosed, setIsClosed] = useState(!!closed);
	className = className || "";

	let primarySize = 5;
	let secondarySize = 50;
	let handleStyle = {
		// width: secondarySize,
		width: "100%",
		height: primarySize,
		// marginBottom: handleMargin,
		// marginTop: -(handleMargin + deltaPosition.y),
		marginLeft: "auto",
		marginRight: "auto",
		marginTop: -primarySize,
		// left: 0,
		// right: 0,
		// bottom: 0,
		// position: "absolute",
	};

	let containerStyle: any = {
		height: isClosed ? 0 : startingValue,
	};

	return (
		<div
			className={`${isClosed ? "closed" : ""} ${className}`}
			style={containerStyle}
		>
			<div
				onClick={() => setIsClosed((isClosed) => !isClosed)}
				style={{ ...handleStyle }}
				className="resizable-panel-handle"
			/>

			{children}
		</div>
	);
}
