import React, { useEffect, useRef, useState } from "react";
import { IRecord } from "./interfaces";

interface IDisplayRecord {
	record: IRecord;
	recordIndex: number;
	numRecords: number;
}

export default function DisplayRecord({
	record,
	recordIndex,
	numRecords,
}: IDisplayRecord) {
	return (
		<div>
			<div>
				{recordIndex + 1} / {numRecords}
			</div>
			<div>{record.Name}</div>
			<div>Times opened: {record.TimesOpened}</div>
			<div>Size: {printSize(record.Size)}</div>
		</div>
	);
}

function printSize(bytes: number) {
	return `${bytes}b`;
}
