import { writable } from 'svelte/store';

import { Notification } from '../types';

const MAX_NOTIFICATIONS = 20;

export const notifications = writable<Notification[]>([]);

export function addNotifications(newNotifications: Notification[]) {
    notifications.update(notifications => {
        const values = [...newNotifications, ...notifications].slice(0, MAX_NOTIFICATIONS);
        values.sort((a, b) => +b.id - a.id);

        return values;
    });
}

export function addNotification(newNotification: Notification) {
    notifications.update(notifications => {
        const values = [newNotification, ...notifications].slice(0, MAX_NOTIFICATIONS);
        values.sort((a, b) => +b.id - a.id);

        return values;
    });
}

export function readNotification(notification: Notification) {
    notifications.update(notifications => {
        const index = notifications.findIndex(n => +n.id === +notification.id);
        if (index !== -1) {
            notifications.splice(index, 1, { ...notification, read: true });
        }

        return notifications;
    });
}
