import { PostNotification as PostNotificationModel, PostNotification } from './models/Notification';
import { TokenData } from './services/sso';
import { token } from './stores/auth';
import { addPost, setPosts } from './stores/posts';
import { Post, NotificationDTO } from './types';
import utils from './utils';
import NotificationPopups from './stores/NotificationPopups';
import Notifications from './stores/Notifications';

interface WsPostCreated {
  readonly event: 'post_created';
  readonly data: {
    readonly item: Post;
  };
}

interface WsNotificationCreated {
  readonly event: 'notification_created';
  readonly data: {
    readonly item: NotificationDTO;
  };
}

type WsEvent = WsPostCreated | WsNotificationCreated;

function isWsEvent(data: unknown): data is WsEvent {
  return typeof (data as WsEvent).event !== undefined;
}

let _token: TokenData | null = null;
token.subscribe((token) => (_token = token));

export class Ws {
  private ws: null | WebSocket = null;
  private keepAliveInterval: null | number = null;

  public constructor(private readonly url: string) {
    this.open();
  }

  private open = () => {
    this.ws = new WebSocket(this.url);
    this.ws.addEventListener('open', this.onOpen);
    this.ws.addEventListener('close', this.onClose);
    this.ws.addEventListener('message', this.onMessage);
    this.ws.addEventListener('error', this.onError);

    const ws = this.ws;
    this.keepAliveInterval = (setInterval(() => {
      if (ws.readyState !== ws.OPEN) {
        return;
      }

      ws.send('keepalive');
    }, 60000) as unknown) as number;
  };

  private close = () => {
    this.ws?.close();
    this.ws = null;

    if (this.keepAliveInterval) {
      clearInterval(this.keepAliveInterval);
      this.keepAliveInterval = null;
    }
  };

  private onOpen = () => {
    // Reload latest posts and notifications after websocket connected.
    window.api?.getLatestPosts().then((posts) => {
      const scroll = utils.isAtBottom();
      if (scroll) {
        setTimeout(utils.scrollToBottom);
      }

      setPosts(posts);
    });

    // Auth required to get notifications.
    if (!window.sso?.loaded) {
      return;
    }

    window.api?.getNotifications().then((notifications) => {
      const notificationModels = notifications.map(PostNotification.createFromDTO);
      Notifications.replaceAll(notificationModels);
    });
  };

  private onClose = (e: CloseEvent) => {
    if (e.code === 1000) {
      return;
    }

    setTimeout(this.open, 10000);
  };

  private onMessage = (e: MessageEvent) => {
    const message = JSON.parse(e.data);
    if (!isWsEvent(message)) {
      return;
    }

    switch (message.event) {
      case 'post_created': {
        const scroll = utils.isAtBottom();
        if (scroll) {
          setTimeout(utils.scrollToBottom);
        }

        addPost(message.data.item);
        break;
      }

      case 'notification_created': {
        if (message.data.item.user_uuid === _token?.user_uuid) {
          const notificationModel = PostNotificationModel.createFromDTO(message.data.item);
          Notifications.add(notificationModel);
          NotificationPopups.add(notificationModel);
        }
        break;
      }
    }
  };

  private onError = () => {
    this.close();
  };
}

export default Ws;
