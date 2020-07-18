import { writable } from 'svelte/store';
import { PostNotification } from '../models/Notification';

const { subscribe, set, update } = writable<PostNotification[]>([]);

export default {
  subscribe,

  replaceAll(notifications: PostNotification[]): void {
    set(notifications);
  },

  add(notification: PostNotification): void {
    update((items) => [notification, ...items]);
  },

  read(id: number): void {
    update((items) =>
      items.map((item) =>
        item.id === id ? (PostNotification.read(item) as PostNotification) : item,
      ),
    );
  },

  remove(id: number): void {
    update((items) => items.filter((item) => item.id !== id));
  },

  removeAll(): void {
    set([]);
  },
};
