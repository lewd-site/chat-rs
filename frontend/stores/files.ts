import { writable } from 'svelte/store';

import { Media } from '../types';

export const mediaBoxFiles = writable<Media[]>([]);
export const mediaBoxFile = writable<null | Media>(null);

const _nsfwMode = localStorage.getItem('settings.nsfw') === 'true';

export const nsfwMode = writable<boolean>(_nsfwMode);

export function setNSFWMode(value: boolean): void {
  nsfwMode.set(value);
  localStorage.setItem('settings.nsfw', `${value}`);
}

export function toggleNSFWMode(): void {
  nsfwMode.update((value) => {
    localStorage.setItem('settings.nsfw', `${!value}`);
    return !value;
  });
}

export const galleryVisible = writable<boolean>(false);

export function showGallery(): void {
  mediaBoxFiles.set([]);
  mediaBoxFile.set(null);
  galleryVisible.set(true);
}

export function hideGallery(): void {
  galleryVisible.set(false);
}

export function toggleGallery(): void {
  galleryVisible.update((value) => {
    if (!value) {
      mediaBoxFiles.set([]);
      mediaBoxFile.set(null);
    }

    return !value;
  });
}

const MAX_FILES = 100;

export const favoriteFiles = writable<File[]>([]);

export function setFavoriteFiles(newFiles: File[]): void {
  const files = newFiles.slice(0, MAX_FILES);
  favoriteFiles.set(files);
}

export const recentFiles = writable<File[]>([]);

export function setRecentFiles(newFiles: File[]): void {
  const files = newFiles.slice(0, MAX_FILES);
  recentFiles.set(files);
}

type ShowOriginalFiles = 'none' | 'gif' | 'all';

let _showOriginalFiles: ShowOriginalFiles = 'gif';
if (localStorage.getItem('settings.original_files') !== null) {
  _showOriginalFiles = localStorage['settings.original_files'];
}

export const showOriginalFiles = writable<ShowOriginalFiles>(_showOriginalFiles);

showOriginalFiles.subscribe((value) => {
  localStorage['settings.original_files'] = value;
});

export const embedTitles = writable<{ [url: string]: string }>({});
