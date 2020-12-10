import React, { useEffect, useState } from "react";
import { Button, Table } from "react-bootstrap";
import { bridge } from "../Utils/Commands";
import { IStartupOptions } from "../Utils/interfaces";

interface IInitializer {
	onInitialize: (initialized: boolean) => any;
}

export function Initializer({ onInitialize }: IInitializer) {
	const [options, setOptions] = useState(null as IStartupOptions | null);

	const selectLocation = (location: string) => {
		(async () => {
			await bridge.initialize({ location });
			onInitialize(true);
		})();
	};

	const addNew = () => {
		(async () => {
			await bridge.openNewFolder();
			onInitialize(true);
		})();
	};

	useEffect(() => {
		(async () => {
			const options = await bridge.getStartupOptions();
			console.log(options);
			setOptions(options);
		})();
	}, []);

	if (!options) {
		return null;
	}

	return (
		<div className="initializer">
			<h1>Select Folder</h1>
			<Table striped bordered hover variant="dark">
				<tbody>
					{options.folders.map((folder) => (
						<tr
							className="clickable"
							onClick={() => selectLocation(folder.location)}
							key={folder.location}
						>
							<td>{getName(folder.location)}</td>
							<td>{folder.location}</td>
						</tr>
					))}
				</tbody>
			</Table>

			<Button style={{ marginLeft: "auto" }} onClick={() => addNew()}>
				Add New
			</Button>
		</div>
	);
}

function getName(location: string) {
	let pieces = location.split(/(\/|\\)/);
	return pieces[pieces.length - 1];
}
