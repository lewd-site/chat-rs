import { writable } from 'svelte/store';

import { File } from '../types';

export const mediaBoxFiles = writable<File[]>([]);
export const mediaBoxFile = writable<null | File>(null);

const _nsfwMode = localStorage.getItem('settings.nsfw') === 'true';

export const nsfwMode = writable<boolean>(_nsfwMode);

export function setNSFWMode(value: boolean) {
    nsfwMode.set(value);
    localStorage.setItem('settings.nsfw', `${value}`);
}

export function toggleNSFWMode() {
    nsfwMode.update(value => {
        localStorage.setItem('settings.nsfw', `${!value}`);
        return !value;
    });
}
