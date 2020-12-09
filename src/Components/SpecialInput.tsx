import { ChangeEvent, useEffect, useRef } from "react";
import { Typeahead } from "react-bootstrap-typeahead";

export interface ISpecialInput {
	className?: string;
	key?: string;
	action: Function;
	// onChange: (event: ChangeEvent<HTMLInputElement>) => void;
	onChange: (value: string) => void;
	value: string;
	actionName?: string;
	prefix?: string;
	extra?: React.ReactElement;

	focusEpoch?: number;
	options?: string[];
}

export function SpecialInput({
	className,
	action,
	onChange,
	value,
	actionName,
	prefix,
	extra,
	focusEpoch, // used to force focus
	options,
}: ISpecialInput) {
	const typeAheadRef = useRef(null as any);

	const thisInput = useRef<HTMLInputElement>(null);
	useEffect(() => {
		if (focusEpoch && thisInput.current) {
			thisInput.current.focus();
			thisInput.current.select();
		}
	}, [thisInput, focusEpoch]);

	return (
		<div className="special-input">
			{prefix ? <button onClick={() => action()}>{prefix}</button> : null}

			{/* TODO: clean up */}
			{options ? (
				<Typeahead
					autoFocus
					ref={typeAheadRef}
					minLength={2}
					id="TODO: make unique id"
					options={options}
					onInputChange={(newValue) => {
						onChange(newValue);
					}}
					onChange={(selection) => {
						if (selection[0]) {
							action(selection[0]);
							typeAheadRef.current.clear();
						}
					}}
					onKeyDown={(event: any) => {
						if ((event.keyCode || event.which) === 13) {
							let value = typeAheadRef.current.getInput().value;
							action(value);
							typeAheadRef.current.clear();
						}
					}}
					// dropup
					open
				/>
			) : (
				<input
					ref={thisInput}
					value={value}
					className={className}
					onChange={(event) => onChange(event.target.value)}
					onKeyPress={(event) => {
						if ((event.keyCode || event.which) === 13) {
							action();
						}
					}}
				/>
			)}

			{actionName ? (
				<button onClick={() => action()}>{actionName}</button>
			) : null}

			{extra}
		</div>
	);
}
