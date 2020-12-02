import React from "react";
import { Accordion, Button, Card } from "react-bootstrap";

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
	undoAdds: (delta: IDelta) => any;
	undoRemoves: (delta: IDelta) => any;
	addTagLine: (tag: string) => any;
	removeTagLine: (tag: string) => any;
}

export function DisplayDeltas({
	deltas,
	undoAdds,
	undoRemoves,
	addTagLine,
	removeTagLine,
}: IDeltas) {
	return (
		<div className="delta-container">
			{deltas.map((delta, i) => (
				<DisplayDelta
					key={delta.id}
					delta={delta}
					undoAdds={undoAdds}
					undoRemoves={undoRemoves}
					addTagLine={addTagLine}
					removeTagLine={removeTagLine}
				/>
			))}
		</div>
	);
}

interface IDisplayDelta {
	delta: IDelta;
	undoAdds: (delta: IDelta) => any;
	undoRemoves: (delta: IDelta) => any;
	addTagLine: (tag: string) => any;
	removeTagLine: (tag: string) => any;
}

function DisplayDelta({
	delta,
	undoAdds,
	undoRemoves,
	addTagLine,
	removeTagLine,
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
							variant={variant}
							size="sm"
							className="truncate fat-child"
						>
							{delta.originalString}
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

interface IDisplayTagLineGroup {
	tag: string;
	variant: "success" | "danger";
	action: (tag: string) => any;
	secondaryAction: (tag: string) => any;
	secondaryTitle: string;
}

export function DisplayTagLineGroup({
	tag,
	variant,
	action,
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
			/>

			<Button
				size="sm"
				variant="dark"
				onClick={() => secondaryAction(tag)}
			>
				{secondaryTitle}
			</Button>
		</div>
	);
}

interface IDisplayTagLine {
	tag: string;
	variant: "success" | "danger";
	action: (tag: string) => any;
}
function DisplayTagLine({ tag, variant, action }: IDisplayTagLine) {
	return (
		<Button
			className="tag truncate"
			size="sm"
			variant={variant}
			onClick={() => action(tag)}
		>
			{tag}
		</Button>
	);
}
