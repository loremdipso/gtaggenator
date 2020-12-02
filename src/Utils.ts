import { useHotkeys } from "react-hotkeys-hook";

export function useHotkeysHelper(shortcut: string, callback: Function, deps?: any[]) {
	let oldShortcut = shortcut;
	if (shortcut.indexOf("numpad") >= 0) {
		shortcut = "*";
	}
	useHotkeys(shortcut, (event: KeyboardEvent) => {
		if (shortcut !== oldShortcut) {
			if (contains(oldShortcut, "alt") !== event.altKey) {
				return;
			}

			if (contains(oldShortcut, "ctrl") !== event.ctrlKey) {
				return;
			}

			if (contains(oldShortcut, "shift") !== event.shiftKey) {
				return;
			}

			if (contains(oldShortcut, "shift") !== event.metaKey) {
				return;
			}

			// TODO: make less hacky
			// console.log(event);;
			let number = oldShortcut[oldShortcut.length - 1];
			if (number !== event.key) {
				return;
			}
		}

		event.preventDefault();
		callback();
	},
		{
			enableOnTags: ["INPUT"],
		},
		deps
	);
}

function contains(toSearch: string, query: string) {
	return toSearch.indexOf(query) >= 0;
}