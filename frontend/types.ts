export interface File {
    readonly id: number;
    readonly md5: string;
    readonly size: number;
    readonly name: string;
    readonly mimetype: string;
    readonly extension: string;
    readonly created_at: string;
    readonly post_id: number;
    readonly width: null | number;
    readonly height: null | number;
    readonly length: null | number;
}

export type EmbedMimeType = 'video/x-coub' | 'video/x-tiktok' | 'video/x-youtube';

export interface Embed {
    readonly id: string;
    readonly name: string;
    readonly mimetype: EmbedMimeType;
    readonly thumbnail_width: number;
    readonly thumbnail_height: number;
    readonly thumbnail_url: string;
    readonly width: number;
    readonly height: number;
    readonly html: string;
    readonly min_width?: number;
    readonly max_width?: number;
}

export type Media = File | Embed;

export interface BBCode {
    readonly type:
    | 'Bold' | 'Italic' | 'Underline' | 'Strike'
    | 'Superscript' | 'Subscript' | 'Code' | 'CodeBlock' | 'Spoiler';
}

export interface Color {
    readonly type: 'Color';
    readonly color: string;
}

export interface RefLink {
    readonly type: 'RefLink';
    readonly id: number;
}

export interface Link {
    readonly type: 'Link';
    readonly url: string;
}

export interface Dice {
    readonly type: 'Dice';
    readonly count: number;
    readonly max: number;
}

export interface Quote {
    readonly type: 'Quote';
}

export interface HTML {
    readonly type: 'HTML';
    readonly content: string;
}

export type Tag = BBCode | Color | RefLink | Link | Dice | Quote | HTML;

interface MarkupText {
    readonly type: 'Text';
    readonly text: string;
}

interface MarkupTag {
    readonly type: 'Tag';
    readonly tag: Tag;
    readonly children: Markup[];
}

export type Markup = MarkupText | MarkupTag;

export interface Post {
    readonly id: number;
    readonly name: string;
    readonly tripcode: string;
    readonly message_raw: string;
    readonly message: Markup[];
    readonly created_at: string;
    readonly files: File[];
    readonly user_uuid: string | null;

    reply_from?: number[];
    embeds?: string[];
}

export interface PostPopup {
    readonly id: number;
    readonly parentPopupId: null | number;
    readonly link: HTMLElement;
    readonly postId: number;
    readonly top: number;
    readonly left: number;
    readonly bottomToTop: boolean;
    readonly rightToLeft: boolean;
    readonly hover: boolean;
    readonly fade: boolean;
    readonly pinned: boolean;
}

export interface SystemNotification {
    readonly type: 'system';
    readonly message: string;
}

export interface PostNotification {
    readonly type: 'post';
    readonly id: number;
    readonly user_uuid: string;
    readonly read: boolean;
    readonly post: Post;
}

export type Notification = SystemNotification | PostNotification;
