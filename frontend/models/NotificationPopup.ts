import { Notification } from './Notification';

let idSeq = 0;

export class NotificationPopup {
  public readonly id: number;
  public readonly lifetime: number;

  public constructor(public readonly notification: Notification) {
    this.id = ++idSeq;
    this.lifetime = 20;
  }

  public static handleTick(notification: NotificationPopup): NotificationPopup {
    return { ...notification, lifetime: notification.lifetime - 1 };
  }

  public static isAlive(notification: NotificationPopup): boolean {
    return notification.lifetime > 0;
  }
}
