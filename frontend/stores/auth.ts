import { writable } from 'svelte/store';

export const showAuthModal = writable<boolean>(false);
