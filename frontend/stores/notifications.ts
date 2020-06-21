import { writable } from 'svelte/store';

import { Notification } from '../types';

export type Notifications = { [key: number]: Notification };

const MAX_NOTIFICATIONS = 20;

export const notifications = writable<Notifications>({});

function hashToArray(notifications: Notifications): Notification[] {
    return Object.values(notifications);
}

function arrayToHash(notifications: Notification[]): Notifications {
    return notifications.reduce((result, notification) => {
        result[notification.id] = notification;
        return result;
    }, {} as Notifications);
}

export function addNotifications(newNotifications: Notification[]) {
    notifications.update(notifications => {
        const values = hashToArray(notifications).concat(newNotifications);
        values.sort((a, b) => +b.id - a.id);

        return arrayToHash(values.slice(0, MAX_NOTIFICATIONS));
    });
}

export function addNotification(newNotification: Notification) {
    notifications.update(notifications => {
        const values = hashToArray(notifications);
        values.push(newNotification);
        values.sort((a, b) => +b.id - a.id);

        return arrayToHash(values.slice(0, MAX_NOTIFICATIONS));
    });
}
