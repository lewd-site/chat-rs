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

export const galleryVisible = writable<boolean>(false);

export function showGallery() {
    galleryVisible.set(true);
}

export function hideGallery() {
    galleryVisible.set(false);
}

export function toggleGallery() {
    galleryVisible.update(value => !value);
}

const MAX_FILES = 100;

export const galleryFiles = writable<File[]>([]);

export function setGalleryFiles(newFiles: File[]) {
    const files = newFiles.slice(0, MAX_FILES);
    galleryFiles.set(files);
}
