import React, { useEffect, useState } from "react";
import { Button, Table } from "react-bootstrap";
import { bridge } from "../Utils/Commands";
import { IStartupFolder, IStartupOptions } from "../Utils/interfaces";

interface IInitializer {
	onInitialize: (initialized: boolean) => any;
}

export function Initializer({ onInitialize }: IInitializer) {
	const [options, setOptions] = useState(null as IStartupOptions | null);

	const selectLocation = (location: string) => {
		// (async () => {
		// await bridge.initialize({ location });
		// we're not going to wait for this async call to complete.
		// Instead we'll let it initialize in the background
		bridge.initialize({ location });
		onInitialize(true);
		// })();
	};

	const addNew = () => {
		(async () => {
			let success = await bridge.openNewFolder();
			if (success) {
				onInitialize(true);
			}
		})();
	};

	const reload = async () => {
		const options = await bridge.getStartupOptions();
		if (options.skip) {
			onInitialize(true);
		} else {
			setOptions(options);
		}
	};

	const removeFolder = async (folder: IStartupFolder) => {
		let success = await bridge.removeFolder({ path: folder.location });
		if (success) {
			reload();
		}
	};

	useEffect(() => {
		reload();
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
							<td width={1}>
								<Button
									variant="danger"
									onClick={(event) => {
										event.stopPropagation();
										removeFolder(folder);
									}}
								>
									X
								</Button>
							</td>
						</tr>
					))}
				</tbody>
			</Table>

			<Button className="centered" onClick={() => addNew()}>
				Add New
			</Button>
		</div>
	);
}

function getName(location: string) {
	let pieces = location.split(/(\/|\\)/);
	return pieces[pieces.length - 1];
}
