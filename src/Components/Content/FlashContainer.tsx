import { useEffect, useRef } from "react";

interface IFlashContainer {
	path: string;
	hidden?: boolean;
}

export function FlashContainer({ path }: IFlashContainer) {
	const ref = useRef(null as HTMLDivElement | null);

	useEffect(() => {
		let container = ref.current;
		if (container) {
			while (container.firstChild) {
				container.removeChild(container.firstChild);
			}

			let ruffle = (window as any).RufflePlayer.newest();
			let player = ruffle.createPlayer();
			(window as any).LastPlayer = player;
			container.appendChild(player);

			container.appendChild(player);
			player.load(path);

			// hacky, but w/e
			let time = 1000;
			let start = Date.now();
			let interval = setInterval(() => {
				if (Date.now() - start > time) {
					clearInterval(interval);
				}
				player.play();
			}, 20);
		}
	}, [path, ref]);

	return <div className="flash-container" ref={ref} />;
}
