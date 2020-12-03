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
	/* <div>
			{recordIndex + 1} / {numRecords}
		</div> */
	return (
		<div>
			<div className="truncate">
				lksdjfkldjfsdlkjf lkj slkfj lsjf lskfjsl ksjj
				{record.Name}
			</div>
			<div className="side-by-side">
				<div>{`Opened ${record.TimesOpened} times`}</div>
				<div>{printSize(record.Size)}</div>
			</div>
		</div>
	);
}

function printSize(bytes: number) {
	return `${bytes}b`;
}
