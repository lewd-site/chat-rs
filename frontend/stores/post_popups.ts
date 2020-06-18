import { writable } from 'svelte/store';

import { PostPopup } from '../types';

export type PostPopups = { [key: number]: PostPopup };

const POPUP_CLOSE_TIME = 200;

let _id = 0;
let _popups: PostPopups = {};

export const popups = writable<PostPopups>({});

popups.subscribe(popups => _popups = popups);

export function getPopup(link: HTMLElement) {
    return Object.values(_popups).find(popup => popup.link === link);
}

export function hasPopup(link: HTMLElement) {
    return !!Object.values(_popups).find(popup => popup.link === link);
}

export function addPopup(
    link: HTMLElement,
    parentPopupId: null | number,
    postId: number,
    top: number,
    left: number,
    bottomToTop: boolean,
    rightToLeft: boolean,
) {
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
    };

    popups.update(popups => ({ ...popups, [id]: popup }));
    setTimeout(() => setPopupFadeById(id, false), 100);
}

export function setPopupHoverById(id: number, hover: boolean) {
    popups.update(popups => {
        if (!_popups[id]) {
            return popups;
        }

        return { ...popups, [id]: { ...popups[id], hover } };
    });
}

export function setPopupFadeById(id: number, fade: boolean) {
    popups.update(popups => {
        if (!_popups[id]) {
            return popups;
        }

        return { ...popups, [id]: { ...popups[id], fade } };
    });
}

export function setPopupHover(link: HTMLElement, hover: boolean) {
    const popup = getPopup(link);
    if (popup) {
        setPopupHoverById(popup.id, hover);
    }
}

function doCheckPopup(id: number) {
    const popup = _popups[id];
    if (!popup) {
        return;
    }

    if (popup.hover) {
        return;
    }

    const children = Object.values(_popups).filter(popup => popup.parentPopupId === +id);
    if (children.some(popup => popup.hover)) {
        return;
    }

    setTimeout(() => {
        if (popup.parentPopupId) {
            doCheckPopup(popup.parentPopupId);
        }
    }, POPUP_CLOSE_TIME);

    setPopupFadeById(popup.id, true);

    setTimeout(() => {
        popups.update(popups => {
            const _popups = { ...popups };
            delete _popups[popup.id];
            return _popups;
        });
    }, 100);
}

export function checkPopupById(id: number) {
    setTimeout(() => doCheckPopup(id), POPUP_CLOSE_TIME);
}

export function checkPopup(link: HTMLElement) {
    const popup = getPopup(link);
    if (popup) {
        checkPopupById(popup.id);
    }
}
