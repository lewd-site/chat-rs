import { Post } from '../types';

export function formatName(post: Post): string {
    if (!post.name.length && !post.tripcode.length) {
        return 'Anonymous';
    }

    return post.name;
}
