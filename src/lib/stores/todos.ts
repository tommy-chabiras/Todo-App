import { writable } from "svelte/store";


function createTodoStore() {
	const todos = writable<Todo[]>([]);

	return 
}

export const todos = createTodoStore();