import { writable } from 'svelte/store';

import { File } from '../types';

export const mediaBoxFile = writable<null | File>(null);
