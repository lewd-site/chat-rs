import { writable } from 'svelte/store';

import { Post } from './types';

type Posts = { [key: number]: Post };

const MAX_POSTS = 100;

export const posts = writable<Posts>({});

export function addPosts(newPosts: Post[]) {
    posts.update(posts => {
        const values = Object.values(posts).concat(newPosts);

        return values.slice(-MAX_POSTS).reduce((result, post) => {
            result[post.id] = post;
            return result;
        }, {} as Posts);
    });
}

export function addPost(newPost: Post) {
    posts.update(posts => {
        const values = Object.values(posts);
        values.push(newPost);

        return values.slice(-MAX_POSTS).reduce((result, post) => {
            result[post.id] = post;
            return result;
        }, {} as Posts);
    });
}

export const mediaBoxFile = writable<null | File>(null);
