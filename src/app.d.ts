declare global {
	interface Todo {
		id: string;
		title: string;
		description: string;
		completed: boolean;
		createdAt: Date;
	}
	type ListFilter = "current" | "completed";
	type ListSort = "date" | "a-z" | "custom";
	type CalendarFilter = "day" | "week" | "month";
	
	interface NavState {
		view: "list" | "calendar";
		list: {
			filter: ListFilter;
			sort: ListSort;
		};
		calendar: {
			filter: CalendarFilter;
		};
	}
}


export {};
