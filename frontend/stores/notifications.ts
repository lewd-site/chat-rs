import { writable } from 'svelte/store';

import { Notification } from '../types';

const MAX_NOTIFICATIONS = 20;

export const notifications = writable<Notification[]>([]);
export const newNotifications = writable<Notification[]>([]);

export function setNotifications(newNotifications: Notification[]) {
    const _notifications = newNotifications.slice(0, MAX_NOTIFICATIONS);
    _notifications.sort((a, b) => +b.id - a.id);
    notifications.set(_notifications);
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

export function removeNotification(notification: Notification) {
    notifications.update(notifications => notifications.filter(n => +n.id !== +notification.id));
}

export function addNewNotification(notification: Notification) {
    newNotifications.update(notifications => [notification, ...notifications]);
    setTimeout(() => removeNewNotification(notification), 10000);
}

export function removeNewNotification(notification: Notification) {
    newNotifications.update(notifications => notifications.filter(n => +n.id !== +notification.id));
}
