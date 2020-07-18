import { writable } from 'svelte/store';

import { mediaBoxFile } from './files';
import { PostPopup, File, Embed, isFile } from '../types';

export type PostPopups = { [key: number]: PostPopup };

const POPUP_CLOSE_TIME = 200;

let _id = 0;
let _popups: PostPopups = {};
let _file: File | Embed | null = null;

export const popups = writable<PostPopups>({});

popups.subscribe((popups) => (_popups = popups));
mediaBoxFile.subscribe((file) => (_file = file));

export function getPopup(link: HTMLElement): PostPopup | undefined {
  return Object.values(_popups).find((popup) => popup.link === link);
}

export function hasPopup(link: HTMLElement): boolean {
  return !!Object.values(_popups).find((popup) => popup.link === link);
}

export function addPopup(
  link: HTMLElement,
  parentPopupId: null | number,
  postId: number,
  top: number,
  left: number,
  bottomToTop: boolean,
  rightToLeft: boolean,
): void {
  if (hasPopup(link)) {
    return;
  }

  const id = ++_id;
  const popup: PostPopup = {
    id,
    parentPopupId,
    link,
    postId,
    top,
    left,
    bottomToTop,
    rightToLeft,
    hover: true,
    fade: true,
    pinned: false,
  };

  popups.update((popups) => ({ ...popups, [id]: popup }));
  setTimeout(() => setPopupFadeById(id, false), 100);
}

export function setPopupHoverById(id: number, hover: boolean): void {
  popups.update((popups) => {
    if (!_popups[id]) {
      return popups;
    }

    return { ...popups, [id]: { ...popups[id], hover } };
  });
}

export function setPopupPinnedById(id: number, pinned: boolean): void {
  popups.update((popups) => {
    if (!_popups[id]) {
      return popups;
    }

    return { ...popups, [id]: { ...popups[id], pinned } };
  });
}

export function setPopupFadeById(id: number, fade: boolean): void {
  popups.update((popups) => {
    if (!_popups[id]) {
      return popups;
    }

    return { ...popups, [id]: { ...popups[id], fade } };
  });
}

export function setPopupHover(link: HTMLElement, hover: boolean): void {
  const popup = getPopup(link);
  if (popup) {
    setPopupHoverById(popup.id, hover);
  }
}

function shouldClosePopup(popup: PostPopup): boolean {
  if (popup.hover || popup.pinned) {
    return false;
  }

  if (_file !== null && isFile(_file) && +_file.post_id === +popup.postId) {
    return false;
  }

  const children = Object.values(_popups).filter(
    (p) => p.parentPopupId && +p.parentPopupId === +popup.id,
  );
  return children.every((p) => shouldClosePopup(p));
}

function doCheckPopup(id: number) {
  const popup = _popups[id];
  if (!popup) {
    return;
  }

  if (!shouldClosePopup(popup)) {
    return;
  }

  setPopupFadeById(popup.id, true);

  setTimeout(() => {
    popups.update((popups) => {
      const _popups = { ...popups };
      delete _popups[popup.id];
      return _popups;
    });
  }, 100);

  setTimeout(() => {
    if (popup.parentPopupId) {
      doCheckPopup(popup.parentPopupId);
    }
  }, POPUP_CLOSE_TIME);
}

export function checkPopupById(id: number): void {
  setTimeout(() => doCheckPopup(id), POPUP_CLOSE_TIME);
}

export function checkPopup(link: HTMLElement): void {
  const popup = getPopup(link);
  if (popup) {
    checkPopupById(popup.id);
  }
}
