import { formatName } from '../components/post';
import { Markup, File, NotificationDTO } from '../types';
import { Posts, posts } from '../stores/posts';
import { token } from '../stores/auth';
import { TokenData } from '../services/sso';

export class Notification {
  public constructor(public readonly createdAt: Date, public readonly isRead: boolean) {}

  private static formatDate(date: Date) {
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');

    return `${day}.${month}.${year}`;
  }

  private static formatTime(date: Date) {
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');

    return `${hours}:${minutes}`;
  }

  public static formatDateTime(date: Date): string {
    const dateNowStr = Notification.formatDate(new Date());
    const dateStr = Notification.formatDate(date);
    const timeStr = Notification.formatTime(date);

    return dateStr === dateNowStr ? timeStr : `${dateStr} ${timeStr}`;
  }

  public static read(notification: Notification): Notification {
    return { ...notification, isRead: true };
  }
}

export class SystemNotification extends Notification {
  public constructor(public readonly message: string, createdAt: Date) {
    super(createdAt, false);
  }
}

let _posts: Posts = {};
posts.subscribe((posts) => (_posts = posts));

let _token: TokenData | null = null;
token.subscribe((token) => (_token = token));

export class PostNotification extends Notification {
  public constructor(
    public readonly id: number,
    public readonly postId: number,
    public readonly name: string,
    public readonly tripcode: string,
    public readonly message: Markup[],
    public readonly files: File[],
    createdAt: Date,
    isRead: boolean,
  ) {
    super(createdAt, isRead);
  }

  private static extractReply(markup: Markup[]): Markup[] {
    let isInReply = false;
    const result: Markup[] = [];
    for (let i = 0; i < markup.length; ++i) {
      const m = markup[i];
      if (m.type === 'Tag' && m.tag.type === 'RefLink') {
        const { id } = m.tag;
        const post = _posts[id];
        const isOwnPost = post && post.user_uuid === _token?.user_uuid;
        if (isOwnPost) {
          isInReply = true;
        } else if (isInReply) {
          break;
        }
      } else if (isInReply) {
        if (m.type === 'Text') {
          result.push({ ...m, text: m.text.trim() });
        } else {
          result.push(m);
        }
      }
    }

    if (!result.length) {
      return markup;
    }

    return result;
  }

  public static createFromDTO(dto: NotificationDTO): PostNotification {
    return new PostNotification(
      dto.id,
      dto.post.id,
      formatName(dto.post),
      dto.post.tripcode,
      PostNotification.extractReply(dto.post.message),
      dto.post.files,
      new Date(dto.post.created_at + 'Z'),
      dto.read,
    );
  }
}
