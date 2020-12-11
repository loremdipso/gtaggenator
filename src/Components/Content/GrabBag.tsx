import React, { useState, useEffect } from "react";
import { bridge } from "../../Utils/Commands";
import { IRecord } from "../../Utils/interfaces";
import ResizablePanel from "../ResizablePanel";

interface IGrabBag {
	record: IRecord;
}
export function GrabBag({ record }: IGrabBag) {
	let [grabBag, setGrabBag] = useState({} as { [key: string]: string });
	const [lastOpenedRecordID, setLastOpenedRecordID] = useState(
		null as number | null
	);

	useEffect(() => {
		if (record && record.RecordID !== lastOpenedRecordID) {
			(async () => {
				setLastOpenedRecordID(record.RecordID);

				// TODO: make sure this doesn't freak out and loop forever
				// TODO: show this somewhere
				// also, should we actually load this all the time?
				try {
					let grabBag = await bridge.getGrabBag({ record: record });
					setGrabBag(grabBag);
				} catch (e) {
					console.log(e);
				}
			})();
		}
	}, [record, lastOpenedRecordID, setGrabBag]);

	if (!Object.keys(grabBag).length) {
		return null;
	}

	return (
		<ResizablePanel startingValue={350} position="right">
			<div className="grabbag-container">
				{Object.keys(grabBag).map((key) => (
					<div key={key} className="grabbag-row">
						<div className="grabbag-key">{key}</div>
						<div className="grabbag-value">{grabBag[key]}</div>
					</div>
				))}
			</div>
		</ResizablePanel>
	);
}
