import { Post, Markup, RefLink } from '../types';

export function formatName(post: Post): string {
    if (!post.name.length && !post.tripcode.length) {
        return 'Anonymous';
    }

    return post.name;
}

export function getRefLink(m: Markup): RefLink | null {
    const tag = m.tags.find(tag => tag.type === 'RefLink');
    if (!tag) {
        return null;
    }

    return tag as RefLink;
}

export function trimStart(m: Markup): Markup {
    return { ...m, text: m.text.replace(/^\s*/, '') };
}

export interface Reply {
    readonly postId: number;
    readonly message: Markup[];
}

export function extractReplies(post: Post): Reply[] {
    const replies: Reply[] = [];
    post.message.forEach(m => {
        const reflink = getRefLink(m);
        if (reflink !== null) {
            replies.push({ postId: reflink.id, message: [] });
        } else if (replies.length > 0) {
            replies[replies.length - 1].message.push(m);
        }
    });

    return replies;
}
