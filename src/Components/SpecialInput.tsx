import { ChangeEvent, useEffect, useRef, useState } from "react";
import { Typeahead } from "react-bootstrap-typeahead";

export interface ISpecialInput {
	key?: string;

	action: Function;
	options: string[];
	// onChange: (value: string) => void;
	// value: string;

	actionName?: string;
	prefix?: string;
	extra?: React.ReactElement;
	focusEpoch?: number;
}

export function SpecialInput({
	action,
	// onChange,
	actionName,
	prefix,
	extra,
	options,
	focusEpoch,
}: ISpecialInput) {
	const typeAheadRef = useRef(null as any);

	useEffect(() => {
		if (focusEpoch && typeAheadRef.current) {
			let input = typeAheadRef.current.getInput();
			input.focus();
			input.select();
		}
	}, [typeAheadRef, focusEpoch]);

	let [, setActionDebouncer] = useState("");
	const doAction = (value: string) => {
		setActionDebouncer((oldValue) => {
			if (value !== oldValue) {
				action(value);
			}
			return value;
		});
	};

	return (
		<div className="special-input">
			{prefix ? <button onClick={() => action()}>{prefix}</button> : null}

			<Typeahead
				autoFocus
				ref={typeAheadRef}
				minLength={2}
				id="TODO: make unique id"
				options={options}
				// onInputChange={(newValue) => {
				// 	onChange(newValue);
				// }}
				onChange={(selection) => {
					if (selection[0]) {
						doAction(selection[0]);
						typeAheadRef.current.clear();
					}
				}}
				onKeyDown={(event: any) => {
					if ((event.keyCode || event.which) === 13) {
						let value = typeAheadRef.current.getInput().value;
						doAction(value);
						typeAheadRef.current.clear();
					}
				}}
				// dropup
				open
			/>

			{actionName ? (
				<button
					onClick={() =>
						action(typeAheadRef.current.getInput().value)
					}
				>
					{actionName}
				</button>
			) : null}

			{extra}
		</div>
	);
}

export interface ISpecialInputSimple {
	className?: string;
	key?: string;
	action: Function;
	onChange: (value: string) => void;
	value: string;
	actionName?: string;
	prefix?: string;
	extra?: React.ReactElement;
	focusEpoch?: number;
}

export function SpecialInputSimple({
	className,
	action,
	onChange,
	value,
	actionName,
	prefix,
	extra,
	focusEpoch,
}: ISpecialInputSimple) {
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

			{actionName ? (
				<button onClick={() => action()}>{actionName}</button>
			) : null}

			{extra}
		</div>
	);
}
