import { writable } from 'svelte/store';

import { Notification, PostNotification } from '../types';

const MAX_NOTIFICATIONS = 20;
export const CLOSE_NEW_NOTIFICATION_DELAY = 20000;

export const notifications = writable<PostNotification[]>([]);
export const newNotifications = writable<Notification[]>([]);

export function setNotifications(newNotifications: PostNotification[]) {
    const _notifications = newNotifications.slice(0, MAX_NOTIFICATIONS);
    _notifications.sort((a, b) => +b.id - a.id);
    notifications.set(_notifications);
}

export function addNotification(newNotification: PostNotification) {
    notifications.update(notifications => {
        const values = [newNotification, ...notifications].slice(0, MAX_NOTIFICATIONS);
        values.sort((a, b) => +b.id - a.id);

        return values;
    });
}

export function readNotification(notification: PostNotification) {
    notifications.update(notifications => {
        const index = notifications.findIndex(n => +n.id === +notification.id);
        if (index !== -1) {
            notifications.splice(index, 1, { ...notification, read: true });
        }

        return notifications;
    });
}

export function removeNotification(notification: PostNotification) {
    notifications.update(notifications => notifications.filter(n => +n.id !== +notification.id));
}

export function addNewNotification(notification: Notification) {
    newNotifications.update(notifications => [notification, ...notifications]);

    let timeout: number | null = null;

    const blur = () => {
        if (timeout) {
            clearTimeout(timeout);
            timeout = null;
        }
    };

    const focus = () => {
        if (!timeout) {
            timeout = setTimeout(() => {
                document.removeEventListener('blur', blur);
                document.removeEventListener('focus', focus);

                removeNewNotification(notification);
            }, CLOSE_NEW_NOTIFICATION_DELAY);
        }
    };

    document.addEventListener('blur', blur);
    document.addEventListener('focus', focus);

    if (document.hidden) {
        blur();
    } else {
        focus();
    }
}

export function removeNewNotification(notification: Notification) {
    newNotifications.update(notifications => notifications.filter(n => n !== notification));
}

export function removeNewNotifications() {
    newNotifications.set([]);
}
