import { writable } from 'svelte/store';

import { Post, RefLink } from './types';

type Posts = { [key: number]: Post };

const MAX_POSTS = 100;

export const posts = writable<Posts>({});

function processPost(post: Post, posts: Post[]): Post {
    post.message.forEach(segment => {
        const refLinkTags = segment.tags.filter(tag => tag.type === 'RefLink') as RefLink[];
        refLinkTags.forEach(tag => {
            const targetPost = posts.find(p => +p.id === +tag.id);
            if (targetPost !== undefined) {
                if (targetPost.reply_from === undefined) {
                    targetPost.reply_from = [post.id];
                } else if (targetPost.reply_from.indexOf(post.id) === -1) {
                    targetPost.reply_from.push(post.id);
                }
            }
        });
    });

    return post;
}

export function setPosts(newPosts: Post[]) {
    newPosts.forEach(newPost => processPost(newPost, newPosts));

    const newPostsHash = newPosts.slice(-MAX_POSTS).reduce((result, post) => {
        result[post.id] = post;
        return result;
    }, {} as Posts);

    posts.set(newPostsHash);
}

export function addPosts(newPosts: Post[]) {
    posts.update(posts => {
        const values = Object.values(posts).concat(newPosts);

        newPosts.forEach(newPost => processPost(newPost, values));

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

        processPost(newPost, values)

        return values.slice(-MAX_POSTS).reduce((result, post) => {
            result[post.id] = post;
            return result;
        }, {} as Posts);
    });
}

export const mediaBoxFile = writable<null | File>(null);
