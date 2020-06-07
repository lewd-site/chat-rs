import { writable } from 'svelte/store';

import { Post } from './types';

type Posts = { [key: number]: Post };

export const posts = writable<Posts>({});

export function addPosts(newPosts: Post[]) {
    const indexedNewPosts = newPosts.reduce((result, post) => {
        result[post.id] = post;
        return result;
    }, {} as Posts);

    posts.update(posts => ({ ...posts, ...indexedNewPosts }));
}

export function addPost(newPost: Post) {
    posts.update(posts => ({ ...posts, [newPost.id]: newPost }));
}
