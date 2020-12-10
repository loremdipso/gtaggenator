import { useEffect, useState } from "react";
import { bridge } from "../Utils/Commands";
import { IStartupFolder, IStartupOptions } from "../Utils/interfaces";

interface IInitializer {
	onInitialize: (initialized: boolean) => any;
}

export function Initializer({ onInitialize }: IInitializer) {
	const [options, setOptions] = useState(null as IStartupOptions | null);

	const doit = (location: string) => {
		(async () => {
			await bridge.initialize({ location });
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
		<div>
			{options.folders.map((folder, i) => (
				<button
					key={folder.location}
					onClick={() => doit(folder.location)}
				>
					{folder.location}
				</button>
			))}
		</div>
	);
}
