import { writable } from 'svelte/store';

import { Post, Markup } from '../types';

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

function updatePost(id: number, callback: (post: Post) => Post) {
    posts.update(posts => {
        const post = posts[id];
        if (typeof post !== 'undefined') {
            return { ...posts, [id]: callback(post) };
        } else {
            return posts;
        }
    });
}

function processMarkup(markup: Markup, post: Post): Markup {
    if (markup.type === 'Text') {
        return markup;
    } else if (markup.type === 'Tag') {
        let { children } = markup;
        children = children.map(m => processMarkup(m, post));

        const { tag } = markup;
        if (tag.type === 'RefLink') {
            setTimeout(() => updatePost(+tag.id, targetPost => {
                const reply_from = typeof targetPost.reply_from !== 'undefined'
                    ? Array.from(new Set([...targetPost.reply_from, post.id]))
                    : [post.id];
                return { ...targetPost, reply_from };
            }));
        } else if (tag.type === 'Link') {
            if (/^(?:https?:\/\/)?(?:www\.)?(?:voca\.ro|vocaroo\.com)\/([A-Za-z0-9]+)$/.test(tag.url)) {
                const matches = tag.url.match(/^(?:https?:\/\/)?(?:www\.)?(?:voca\.ro|vocaroo\.com)\/([A-Za-z0-9]+)$/);
                const html = `<iframe class="markup_vocaroo" width="300" height="60" src="https://vocaroo.com/embed/${matches![1]}" frameborder="0"></iframe>`;

                return { type: 'Tag', tag: { type: 'HTML', content: html }, children: [] };
            } else if (
                /^(?:https?:\/\/)?(?:www\.)?coub\.com\/view\//i.test(tag.url) ||
                /^(?:https?:\/\/)?(?:www\.)?(?:tiktok\.com)\/@([0-9a-z_-]+)\/video\/(?:\d+)/i.test(tag.url) ||
                /^(?:https?:\/\/)?(?:www\.)?(?:youtube\.com\/(?:watch|embed|v)|youtu\.be\/)/i.test(tag.url)
            ) {
                if (post.embeds === undefined) {
                    post.embeds = [tag.url];
                } else {
                    post.embeds.push(tag.url);
                }
            }
        }

        return { ...markup, children };
    } else {
        return markup;
    }
}

function processPost(post: Post): Post {
    const message = post.message.map(m => processMarkup(m, post));
    return { ...post, message };
}

export function addPost(post: Post) {
    posts.update(posts => {
        return { ...posts, [post.id]: processPost(post) }
    });
}

export function addPosts(newPosts: Post[]) {
    posts.update(posts => {
        const result = { ...posts };
        newPosts.forEach(p => result[p.id] = processPost(p));
        return result;
    });
}

export function setPosts(newPosts: Post[]) {
    posts.set({});
    addPosts(newPosts);
}

export function unloadOldPosts() {
    posts.update(posts => {
        const values = hashToArray(posts);
        return arrayToHash(values.slice(-MAX_POSTS));
    });
}

export const visiblePosts = writable<number[]>([]);
