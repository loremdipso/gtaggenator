import React, { useEffect } from "react";
import { Accordion, Button, Card } from "react-bootstrap";
import { bridge, CACHE_KEYS } from "../Utils/Commands";

export interface ISearch {
	id: number;
	text: string;
	favorite: boolean;
}

const getSearchID = (() => {
	let id = 0;
	return () => id++;
})();

interface IDisplaySearches {
	searches: ISearch[];
	setSearches: any;
	action: any;
}

export function DisplaySearches({
	searches,
	setSearches,
	action,
}: IDisplaySearches) {
	const removeSearch = (searchToRemove: ISearch) => {
		setSearches((searches: ISearch[]) =>
			searches.filter((search) => search.id !== searchToRemove.id)
		);
	};

	const toggleFavorite = (searchToToggle: ISearch) => {
		setSearches((searches: ISearch[]) =>
			appendSearchImmutable(searches, {
				...searchToToggle,
				favorite: !searchToToggle.favorite,
			})
		);
	};

	return (
		<div className="delta-container">
			{searches.map((search, i) => (
				<DisplaySearch
					key={search.id}
					search={search}
					toggleFavorite={toggleFavorite}
					removeSearch={removeSearch}
					action={action}
				/>
			))}
		</div>
	);
}

interface IDisplaySearch {
	search: ISearch;
	toggleFavorite: (search: ISearch) => any;
	removeSearch: (search: ISearch) => any;
	action: (search: string) => any;
}

function DisplaySearch({
	search,
	toggleFavorite,
	removeSearch,
	action,
}: IDisplaySearch) {
	let variant = "secondary";

	return (
		<Card>
			<Card.Header>
				<div className="tag-header">
					<Button
						onClick={() => action(search.text)}
						variant={variant}
						size="sm"
						className="truncate fat-child"
					>
						{search.text}
					</Button>
					<Button
						variant="dark"
						onClick={() => toggleFavorite(search)}
						size="sm"
						style={{ minWidth: 25 }}
					>
						{search.favorite ? "*" : "O"}
					</Button>
					<Button
						variant="dark"
						onClick={() => removeSearch(search)}
						size="sm"
						style={{ minWidth: 25 }}
					>
						X
					</Button>
				</div>
			</Card.Header>
		</Card>
	);
}

export function appendSearchImmutable(
	searches: ISearch[],
	newSearch: ISearch
): ISearch[] {
	if (newSearch.favorite) {
		return [
			// pre-pend newSearch
			newSearch,
			...searches.filter((search) => search.id !== newSearch.id),
		];
	} else {
		let newSearches = [];
		let didSet = false;
		for (let oldDelta of searches) {
			if (!didSet && !oldDelta.favorite) {
				newSearches.push(newSearch);
				didSet = true;
			}
			if (oldDelta.id !== newSearch.id) {
				newSearches.push(oldDelta);
			}
		}

		if (!didSet) {
			newSearches.push(newSearch);
		}

		return newSearches;
	}
}

export function fixSearches(searches: ISearch[]) {
	for (let search of searches) {
		search.id = getSearchID();
	}
}

const MAX_SEARCHES = 10;
export function addSearch(
	setSearches: Function,
	searches: ISearch[],
	newSearch: string
) {
	newSearch = newSearch.trim();
	if (newSearch.length === 0) {
		return false;
	}

	for (let search of searches) {
		if (search.text == newSearch) {
			return false;
		}
	}

	let search: ISearch = {
		id: getSearchID(),
		text: newSearch,
		favorite: false,
	};

	setSearches((searches: ISearch[]) =>
		appendSearchImmutable(searches, search).slice(0, MAX_SEARCHES)
	);

	return true;
}
