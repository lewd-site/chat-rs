import { writable } from 'svelte/store';
import { Post } from './types';

export const posts = writable<Post[]>([]);
