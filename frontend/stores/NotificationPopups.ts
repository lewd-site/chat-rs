import { writable } from 'svelte/store';
import { Notification, PostNotification } from '../models/Notification';
import { NotificationPopup } from '../models/NotificationPopup';

const { subscribe, set, update } = writable<NotificationPopup[]>([]);

export default {
  subscribe,

  add(notification: Notification): number {
    const popup = new NotificationPopup(notification);
    update((items) => [...items, popup]);
    return popup.id;
  },

  remove(id: number): void {
    update((items) => items.filter((item) => item.id !== id));
  },

  removeByNotificationId(id: number): void {
    update((items) =>
      items.filter((item) => {
        if (item.notification instanceof PostNotification) {
          return item.notification.id !== id;
        }

        return true;
      }),
    );
  },

  removeAll(): void {
    set([]);
  },

  handleTick(): void {
    update((items) => {
      return items.map(NotificationPopup.handleTick).filter(NotificationPopup.isAlive);
    });
  },
};
