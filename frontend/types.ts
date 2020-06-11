export interface File {
    readonly id: number,
    readonly md5: string,
    readonly size: number,
    readonly name: string,
    readonly mimetype: string,
    readonly extension: string,
    readonly created_at: string,
    readonly post_id: number,
    readonly width: null | number,
    readonly height: null | number,
    readonly length: null | number,
}

export interface Post {
    readonly id: number;
    readonly name: string;
    readonly tripcode: string;
    readonly message: string;
    readonly created_at: string;
    readonly files: File[];
}
