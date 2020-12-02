export interface IDelta {
	added: string[];
	removed: string[];
	originalString: string;
	id: number;
}

const getDeltaID = (() => {
	let id = 0;
	return () => id++;
})();

interface IDeltas {
	deltas: IDelta[];
	undo: (delta: IDelta) => any;
	redo: (delta: IDelta) => any;
	addTagLine: (tag: string) => any;
	removeTagLine: (tag: string) => any;
}

export function DisplayDeltas({
	deltas,
	undo,
	redo,
	addTagLine,
	removeTagLine,
}: IDeltas) {
	return (
		<div>
			{deltas.map((delta, i) => (
				<div className="delta">
					<button onClick={() => addTagLine(delta.originalString)}>
						{delta.originalString}
					</button>
					{delta.added.length ? (
						<ul className="added">
							{delta.added.map((tag) => (
								<li key={tag}>
									{" "}
									{tag}
									<button onClick={() => removeTagLine(tag)}>
										Remove
									</button>
									<button onClick={() => addTagLine(tag)}>
										Add
									</button>
								</li>
							))}
						</ul>
					) : null}

					{delta.removed.length ? (
						<ul className="removed">
							{delta.removed.map((tag) => (
								<li key={tag}>
									{tag}
									<button onClick={() => removeTagLine(tag)}>
										Remove
									</button>
									<button onClick={() => addTagLine(tag)}>
										Add
									</button>
								</li>
							))}
						</ul>
					) : null}

					<button onClick={() => undo(delta)}>Undo</button>
					<button onClick={() => redo(delta)}>Redo</button>
				</div>
			))}
		</div>
	);
}

export function getDelta(
	newTags: string[],
	oldTags: string[],
	originalString: string
): IDelta {
	// Keeps the order of tags that are in both oldTags and newTags,
	// but keep the tags that are just in newTags at the top
	let removed = [];
	for (let tag of oldTags) {
		if (newTags.indexOf(tag) === -1) {
			removed.push(tag);
		}
	}

	let added = [];
	for (let tag of newTags) {
		if (oldTags.indexOf(tag) === -1) {
			added.push(tag);
		}
	}

	return { added, removed, originalString, id: getDeltaID() };
}
