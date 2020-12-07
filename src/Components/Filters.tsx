import React, { useEffect } from "react";
import { Button, Form } from "react-bootstrap";
import { UpDown } from "./UpDown";

interface ISearchOption {
	display: string;
	command: string;
	type: "filter" | "sorter" | "none";
	valueType?: "string" | "number";
	id: number;
}

const getFilterID = (() => {
	let id = 0;
	return () => id++;
})();

let id = 0;
let type: "filter" | "sorter" = "filter";
const baseFilters: ISearchOption[] = [
	{ display: "None", command: "", id: id++, type: "none" },
	{
		display: "Search",
		command: "-sort search",
		id: id++,
		valueType: "string",
		type,
	},
	{
		display: "Untouched",
		command: "-sort untouched",
		id: id++,
		type,
	},
	{ display: "Touched", command: "-sort touched", id: id++, type },
	{ display: "Seen", command: "-sort seen", id: id++, type },
	{ display: "Unseen", command: "-sort unseen", id: id++, type },
	{
		display: "Limit",
		command: "-sort limit",
		valueType: "number",
		id: id++,
		type,
	},
];

id = 0;
type = "sorter";
const baseSorters: ISearchOption[] = [
	{ display: "None", command: "", id: id++, type: "none" },
	{
		display: "Most Tags",
		command: "-sort most_tags",
		id: id++,
		type,
	},
	{
		display: "Fewest Tags",
		command: "-sort fewest_tags",
		id: id++,
		type,
	},
	{ display: "Largest", command: "-sort largest", id: id++, type },
	{
		display: "Smallest",
		command: "-sort smallest",
		id: id++,
		type,
	},
	{
		display: "Alphabetial",
		command: "-sort alpha",
		id: id++,
		type,
	},
	{
		display: "Random",
		command: "-sort random",
		id: id++,
		type,
	},
];

export interface IFilter {
	base: ISearchOption; // immutable core
	type: "filter" | "sorter" | "none";
	value?: number | string;
	id: number;
}

interface IShowFilters {
	filters: IFilter[];
	setFilters: any;
}

export function DisplayFilters({ filters, setFilters }: IShowFilters) {
	useEffect(() => {
		if (filters.length === 0) {
			setFilters(() => [getEmptyFilter()]);
		}
	}, [filters, setFilters]);

	const removeFilter = (filterToRemove: IFilter) => {
		setFilters((filters: IFilter[]) =>
			filters.filter((filter) => filter.id !== filterToRemove.id)
		);
	};

	const addFilter = (newFilter: IFilter) => {
		setFilters([...filters, newFilter]);
	};

	const modifyFilter = (newFilter: IFilter) => {
		setFilters((filters: IFilter[]) =>
			filters.map((filter) =>
				filter.id === newFilter.id ? newFilter : filter
			)
		);
	};

	const moveRecord = (oldIndex: number, newIndex: number) => {
		let newFilters = [...filters]; // clone, b/c we're functional
		let temp = newFilters[oldIndex];
		newFilters[oldIndex] = newFilters[newIndex];
		newFilters[newIndex] = temp;
		setFilters(newFilters);
	};

	const moveUp = (filterToMove: IFilter) => {
		let oldIndex = getIndex(filters, filterToMove);
		if (oldIndex >= 1) {
			moveRecord(oldIndex, oldIndex - 1);
		}
	};

	const moveDown = (filterToMove: IFilter) => {
		let oldIndex = getIndex(filters, filterToMove);
		if (oldIndex >= 0 && oldIndex < filters.length - 1) {
			moveRecord(oldIndex, oldIndex + 1);
		}
	};

	return (
		<div>
			{filters.map((filter) => (
				<div className="filter-row" key={filter.id}>
					{filters.length > 1 ? (
						<UpDown
							upEvent={() => moveUp(filter)}
							downEvent={() => moveDown(filter)}
						/>
					) : null}

					{filter.type !== "sorter" ? (
						<DropdownHelper
							modifyFilter={modifyFilter}
							type="filter"
							filter={filter}
						/>
					) : null}

					{filter.type !== "filter" ? (
						<DropdownHelper
							modifyFilter={modifyFilter}
							type="sorter"
							filter={filter}
						/>
					) : null}

					{filter.base.valueType ? (
						filter.base.valueType === "number" ? (
							<input
								value={filter.value || 0}
								type="number"
								onChange={(ev) =>
									modifyFilter({
										...filter,
										value: parseInt(ev.target.value) || 0,
									})
								}
							/>
						) : (
							<input
								value={filter.value}
								onChange={(ev) =>
									modifyFilter({
										...filter,
										value: ev.target.value,
									})
								}
							/>
						)
					) : null}

					<Button
						variant="success"
						size="sm"
						onClick={() => addFilter(getEmptyFilter())}
					>
						+
					</Button>

					<Button
						variant="danger"
						size="sm"
						onClick={() => removeFilter(filter)}
					>
						X
					</Button>
				</div>
			))}
		</div>
	);
}

interface IDropdownHelper {
	modifyFilter: Function;
	type: "filter" | "sorter";
	filter: IFilter;
}

function DropdownHelper({ modifyFilter, type, filter }: IDropdownHelper) {
	let constArr = type === "filter" ? baseFilters : baseSorters;
	return (
		<Form.Control
			as="select"
			key={filter.id}
			custom
			size="sm"
			defaultValue={filter.type === type ? filter.base.id : undefined}
			onChange={(ev: any) => {
				let base = constArr[parseInt(ev.target.value)];
				modifyFilter({
					...filter,
					base,
					type: base.type,
				});
			}}
		>
			{constArr.map((base) => (
				<option value={base.id} key={base.id}>
					{base.display}
				</option>
			))}
		</Form.Control>
	);
}

function getDefaultFilter(): IFilter {
	return {
		base: baseFilters[1], // TODO: make less hacky
		type: "filter",
		id: getFilterID(),
	};
}

function getEmptyFilter(): IFilter {
	return {
		base: baseFilters[0],
		type: "none",
		id: getFilterID(),
	};
}

function getIndex(filters: IFilter[], filterToMove: IFilter) {
	return filters.findIndex((el) => el.id === filterToMove.id);
}
