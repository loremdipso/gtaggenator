import { useEffect, useRef, useState } from "react";
import { useHotkeysHelper } from "../../Utils/Hotkeys";

interface IImageContainer {
	path: string;
	hidden?: boolean;
}

export function ImageContainer({ path, hidden }: IImageContainer) {
	const [fit, setFit] = useState("fit-best" as "fit-best" | "fit-width");
	useHotkeysHelper("alt+f", () => {
		setFit((fit) => {
			if (fit === "fit-best") {
				return "fit-width";
			} else {
				return "fit-best";
			}
		});
	});

	return (
		<div className={`image-container ${fit} ${hidden ? "hidden" : ""}`}>
			<img src={path} alt="content" />
		</div>
	);
}
