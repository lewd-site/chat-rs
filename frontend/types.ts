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

interface BBCode {
    readonly type:
    | 'Bold' | 'Italic' | 'Underline' | 'Strike'
    | 'Superscript' | 'Subscript' | 'Code' | 'Spoiler';
}

interface RefLink {
    readonly type: 'RefLink';
    readonly id: number;
}

export type Tag = BBCode | RefLink;

export interface Markup {
    readonly text: string;
    readonly tags: Tag[];
}

export interface Post {
    readonly id: number;
    readonly name: string;
    readonly tripcode: string;
    readonly message_raw: string;
    readonly message: Markup,
    readonly created_at: string;
    readonly files: File[];
}
