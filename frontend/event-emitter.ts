export type Listener = (...args: unknown[]) => void;
export type ListenerCollection = { [event: string]: Listener[] };
export type Unsubscriber = () => void;

export class EventEmitter {
  private readonly listeners: ListenerCollection = {};

  public subscribe(event: string, listener: Listener): Unsubscriber {
    if (this.listeners[event]) {
      this.listeners[event].push(listener);
    } else {
      this.listeners[event] = [listener];
    }

    return () => this.unsubscribe(event, listener);
  }

  public dispatch(event: string, ...args: unknown[]): void {
    if (!this.listeners[event]) {
      return;
    }

    this.listeners[event].forEach((l) => l.apply(this, args));
  }

  public unsubscribe(event: string, listener: Listener): void {
    if (!this.listeners[event]) {
      return;
    }

    this.listeners[event] = this.listeners[event].filter((l) => l !== listener);
  }

  public unsubscribeAll(event: string): void {
    if (!this.listeners[event]) {
      return;
    }

    delete this.listeners[event];
  }
}

export default EventEmitter;
