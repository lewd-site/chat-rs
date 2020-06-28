import { writable } from 'svelte/store';

import { Media } from '../types';

export const mediaBoxFiles = writable<Media[]>([]);
export const mediaBoxFile = writable<null | Media>(null);

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
