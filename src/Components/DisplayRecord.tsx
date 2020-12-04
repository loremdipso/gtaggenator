import { IRecord } from "../interfaces";
import { InfoCircleFill } from "react-bootstrap-icons";
import React from "react";
import { OverlayTrigger, Table, Tooltip } from "react-bootstrap";

interface IDisplayRecord {
	record: IRecord;
}

export default function DisplayRecord({ record }: IDisplayRecord) {
	return (
		<div>
			<div className="side-by-side">
				<div className="truncate">{record.Name}</div>

				<OverlayTrigger
					placement="right"
					overlay={(props: any) => (
						<div {...props}>
							<Table className="my-fancy-table">
								<tbody>
									<tr>
										<td>Added</td>
										<td>{record.DateAdded}</td>
									</tr>
									<tr>
										<td>Created</td>
										<td>{record.DateCreated}</td>
									</tr>
									<tr>
										<td>Last Seen</td>
										<td>{record.DateLastAccessed}</td>
									</tr>
									<tr>
										<td>Location</td>
										<td>{record.Location}</td>
									</tr>
								</tbody>
							</Table>
						</div>
					)}
				>
					<InfoCircleFill />
				</OverlayTrigger>
			</div>
			<div className="side-by-side">
				<div>{`Opened ${record.TimesOpened} times`}</div>
				<div>{printSize(record.Size)}</div>
			</div>
		</div>
	);
}

// borrowed from https://stackoverflow.com/a/14919494/13707438
/**
 * Format bytes as human-readable text.
 *
 * @param bytes Number of bytes.
 * @param si True to use metric (SI) units, aka powers of 1000. False to use
 *           binary (IEC), aka powers of 1024.
 * @param dp Number of decimal places to display.
 *
 * @return Formatted string.
 */
function printSize(bytes: number, si = false, dp = 1) {
	const thresh = si ? 1000 : 1024;

	if (Math.abs(bytes) < thresh) {
		return bytes + " B";
	}

	const units = si
		? ["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"]
		: ["KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
	let u = -1;
	const r = 10 ** dp;

	do {
		bytes /= thresh;
		++u;
	} while (
		Math.round(Math.abs(bytes) * r) / r >= thresh &&
		u < units.length - 1
	);

	return bytes.toFixed(dp) + " " + units[u];
}