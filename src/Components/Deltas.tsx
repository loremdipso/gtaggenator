import React, { useEffect } from "react";
import { Accordion, Button, Card } from "react-bootstrap";
import { bridge, CACHE_KEYS } from "../Utils/Commands";

export interface IDelta {
	added: string[];
	removed: string[];
	originalString: string;
	favorite: boolean;
	id: number;
}

const getDeltaID = (() => {
	let id = 0;
	return () => id++;
})();

interface IDeltas {
	deltas: IDelta[];
	// setDeltas: (deltas: IDelta[]) => any;
	setDeltas: any;
	undoAdds: (delta: IDelta) => any;
	undoRemoves: (delta: IDelta) => any;
	addTagLine: (tag: string) => any;
	removeTagLine: (tag: string) => any;
	onDoubleClick?: (tag: string) => any;
}

export function DisplayDeltas({
	deltas,
	setDeltas,
	undoAdds,
	undoRemoves,
	addTagLine,
	removeTagLine,
	onDoubleClick,
}: IDeltas) {
	const removeDelta = (deltaToRemove: IDelta) => {
		setDeltas((deltas: IDelta[]) =>
			deltas.filter((delta) => delta.id !== deltaToRemove.id)
		);
	};

	const toggleFavorite = (deltaToToggle: IDelta) => {
		setDeltas((deltas: IDelta[]) =>
			appendDeltaImmutable(deltas, {
				...deltaToToggle,
				favorite: !deltaToToggle.favorite,
			})
		);
	};

	return (
		<div className="delta-container">
			{deltas.map((delta, i) => (
				<DisplayDelta
					key={delta.id}
					delta={delta}
					undoAdds={undoAdds}
					toggleFavorite={toggleFavorite}
					undoRemoves={undoRemoves}
					removeDelta={removeDelta}
					addTagLine={addTagLine}
					removeTagLine={removeTagLine}
					onDoubleClick={onDoubleClick}
				/>
			))}
		</div>
	);
}

interface IDisplayDelta {
	delta: IDelta;
	undoAdds: (delta: IDelta) => any;
	undoRemoves: (delta: IDelta) => any;
	toggleFavorite: (delta: IDelta) => any;
	removeDelta: (delta: IDelta) => any;
	addTagLine: (tag: string) => any;
	removeTagLine: (tag: string) => any;
	onDoubleClick?: (tag: string) => any;
}

function DisplayDelta({
	delta,
	undoAdds,
	undoRemoves,
	toggleFavorite,
	removeDelta,
	addTagLine,
	removeTagLine,
	onDoubleClick,
}: IDisplayDelta) {
	let variant = "secondary";
	if (delta.added.length && !delta.removed.length) {
		variant = "success";
	} else if (delta.removed.length && !delta.added.length) {
		variant = "danger";
	}

	let shouldShow = delta.added.length + delta.removed.length > 1;

	return (
		<Accordion defaultActiveKey={shouldShow ? "0" : "100"}>
			<Card>
				<Card.Header>
					<div className="tag-header">
						{delta.added.length > 0 &&
						delta.removed.length === 0 ? (
							<Button
								variant="dark"
								onClick={() => undoAdds(delta)}
								size="sm"
								style={{ minWidth: 50 }}
							>
								-
							</Button>
						) : null}

						{delta.removed.length > 0 &&
						delta.added.length === 0 ? (
							<Button
								variant="dark"
								onClick={() => undoRemoves(delta)}
								size="sm"
								style={{ minWidth: 50 }}
							>
								+
							</Button>
						) : null}

						{/* {shouldShow ? (
							<Accordion.Toggle eventKey="0">
								{">"}
							</Accordion.Toggle>
						) : null} */}

						<Button
							onClick={() => addTagLine(delta.originalString)}
							onDoubleClick={() =>
								onDoubleClick &&
								onDoubleClick(delta.originalString)
							}
							variant={variant}
							size="sm"
							className="truncate fat-child"
						>
							{delta.originalString}
						</Button>

						<Button
							variant="dark"
							onClick={() => toggleFavorite(delta)}
							size="sm"
							style={{ minWidth: 25 }}
						>
							{delta.favorite ? "*" : "O"}
						</Button>
						<Button
							variant="dark"
							onClick={() => removeDelta(delta)}
							size="sm"
							style={{ minWidth: 25 }}
						>
							X
						</Button>
					</div>
				</Card.Header>

				<Accordion.Collapse eventKey="0">
					<Card.Body>
						<div className="delta-container">
							<div className="tag-container">
								{delta.added.length
									? delta.added.map((tag) => (
											<DisplayTagLineGroup
												tag={tag}
												key={tag}
												variant="success"
												action={addTagLine}
												secondaryAction={removeTagLine}
												secondaryTitle="-"
											/>
									  ))
									: null}

								{delta.removed.length
									? delta.removed.map((tag) => (
											<DisplayTagLineGroup
												tag={tag}
												key={tag}
												variant="danger"
												action={removeTagLine}
												secondaryAction={addTagLine}
												secondaryTitle="+"
											/>
									  ))
									: null}
							</div>
						</div>
					</Card.Body>
				</Accordion.Collapse>
			</Card>
		</Accordion>
	);
}

export function createDelta(
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

	return {
		added,
		removed,
		originalString,
		favorite: false,
		id: getDeltaID(),
	};
}

interface IDisplayTagLineGroup {
	tag: string;
	variant: "success" | "danger" | "primary" | "secondary";
	action: (tag: string) => any;
	rightClickAction?: (tag: string) => any;
	secondaryAction?: (tag: string) => any;
	secondaryTitle?: string;
}

export function DisplayTagLineGroup({
	tag,
	variant,
	action,
	rightClickAction,
	secondaryAction,
	secondaryTitle,
}: IDisplayTagLineGroup) {
	return (
		<div>
			<DisplayTagLine
				tag={tag}
				key={tag}
				variant={variant}
				action={action}
				rightClickAction={rightClickAction}
			/>

			{secondaryAction ? (
				<Button
					size="sm"
					variant="dark"
					onClick={() => secondaryAction(tag)}
				>
					{secondaryTitle}
				</Button>
			) : null}
		</div>
	);
}

interface IDisplayTagLine {
	tag: string;
	variant: "success" | "danger" | "primary" | "secondary";
	action: (tag: string) => any;
	rightClickAction?: (tag: string) => any;
}
function DisplayTagLine({
	tag,
	variant,
	action,
	rightClickAction,
}: IDisplayTagLine) {
	return (
		<Button
			className="tag truncate"
			size="sm"
			variant={variant}
			onClick={() => action(tag)}
			onContextMenu={(event) => {
				event.preventDefault();
				rightClickAction && rightClickAction(tag);
				return false;
			}}
		>
			{tag}
		</Button>
	);
}

export function appendDeltaImmutable(
	deltas: IDelta[],
	newDelta: IDelta
): IDelta[] {
	if (newDelta.favorite) {
		return [
			newDelta,
			...deltas.filter((delta) => delta.id !== newDelta.id),
		];
	} else {
		let newDeltas = [];
		let didSet = false;
		for (let oldDelta of deltas) {
			if (!didSet && !oldDelta.favorite) {
				newDeltas.push(newDelta);
				didSet = true;
			}
			if (oldDelta.id !== newDelta.id) {
				newDeltas.push(oldDelta);
			}
		}

		if (!didSet) {
			newDeltas.push(newDelta);
		}

		return newDeltas;
	}
}

export function fixDeltas(deltas: IDelta[]) {
	for (let delta of deltas) {
		delta.id = getDeltaID();
	}
}
