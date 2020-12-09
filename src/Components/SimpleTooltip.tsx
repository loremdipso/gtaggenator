import React from "react";
import { Tooltip } from "react-bootstrap";

export function SimpleTooltip(text: string) {
	return (props: any) => <Tooltip {...props}>{text}</Tooltip>;
}
