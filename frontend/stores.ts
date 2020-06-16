import { writable } from 'svelte/store';

import { Post, RefLink } from './types';

export type Posts = { [key: number]: Post };

const MAX_POSTS = 100;

export const posts = writable<Posts>({});

function hashToArray(posts: Posts): Post[] {
    return Object.values(posts);
}

function arrayToHash(posts: Post[]): Posts {
    return posts.reduce((result, post) => {
        result[post.id] = post;
        return result;
    }, {} as Posts);
}

function processPost(post: Post, allPosts: Post[]) {
    post.message.forEach(segment => {
        const refLinkTags = segment.tags.filter(tag => tag.type === 'RefLink') as RefLink[];
        refLinkTags.forEach(tag => {
            const targetPost = allPosts.find(p => +p.id === +tag.id);
            if (targetPost !== undefined) {
                if (targetPost.reply_from === undefined) {
                    targetPost.reply_from = [post.id];
                } else if (targetPost.reply_from.indexOf(post.id) === -1) {
                    targetPost.reply_from.push(post.id);
                }
            }
        });
    });
}

function processPosts(posts: Post[], allPosts: Post[]) {
    posts.forEach(post => processPost(post, allPosts));
}

export function setPosts(newPosts: Post[]) {
    newPosts.forEach(newPost => processPost(newPost, newPosts));
    posts.set(arrayToHash(newPosts));
}

export function addPosts(newPosts: Post[]) {
    posts.update(posts => {
        const values = hashToArray(posts).concat(newPosts);
        values.sort((a, b) => +a.id - b.id);

        processPosts(newPosts, values);

        return arrayToHash(values);
    });
}

export function addPost(newPost: Post) {
    posts.update(posts => {
        const values = hashToArray(posts);
        values.push(newPost);
        values.sort((a, b) => +a.id - b.id);

        processPost(newPost, values)

        return arrayToHash(values);
    });
}

export function unloadOldPosts() {
    posts.update(posts => {
        const values = hashToArray(posts);
        return arrayToHash(values.slice(-MAX_POSTS));
    });
}

export const mediaBoxFile = writable<null | File>(null);

export const authModal = writable<boolean>(false);
