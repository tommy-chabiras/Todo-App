import { writable } from "svelte/store";

const defaultNav: NavState = {
	view: "list",
	list: {
		filter: "current",
		sort: "date"
	},
	calendar: {
		filter: "week"
	}
};

export const nav = writable<NavState>(defaultNav);
