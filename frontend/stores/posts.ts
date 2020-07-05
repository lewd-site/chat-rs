import { writable } from 'svelte/store';

import { RefLink, Post, Link, Embed, Markup } from '../types';

export type Posts = { [key: number]: Post };

const MAX_POSTS = 100;
const POPUP_HEADER_PADDIGN = 24;

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

function triggerPostUpdate(post: Post) {
    posts.update(posts => ({ ...posts, [post.id]: post }));
}

function processMarkup(markup: Markup, post: Post, allPosts: Post[]): Markup {
    if (markup.type === 'Text') {
        return markup;
    }
    else if (markup.type === 'Tag') {
        let { children } = markup;
        children = children.map(m => processMarkup(m, post, allPosts));

        const { tag } = markup;
        if (tag.type === 'RefLink') {
            const targetPost = allPosts.find(p => +p.id === +tag.id);
            if (targetPost !== undefined) {
                if (targetPost.reply_from === undefined) {
                    targetPost.reply_from = [post.id];
                } else if (targetPost.reply_from.indexOf(post.id) === -1) {
                    targetPost.reply_from.push(post.id);
                }
            }
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

function processPost(post: Post, allPosts: Post[]): Post {
    const message = post.message.map(m => processMarkup(m, post, allPosts));
    return { ...post, message };
}

function processPosts(posts: Post[], allPosts: Post[]): Post[] {
    return posts.map(post => processPost(post, allPosts));
}

export function setPosts(newPosts: Post[]) {
    posts.set(arrayToHash(newPosts.map(newPost => processPost(newPost, newPosts))));
}

export function addPosts(newPosts: Post[]) {
    posts.update(posts => {
        let values = hashToArray(posts);
        values = values.concat(processPosts(newPosts, [...values, ...newPosts]));
        values.sort((a, b) => +a.id - b.id);

        return arrayToHash(values);
    });
}

export function addPost(newPost: Post) {
    posts.update(posts => {
        const values = hashToArray(posts);
        values.push(processPost(newPost, values));
        values.sort((a, b) => +a.id - b.id);

        return arrayToHash(values);
    });
}

export function unloadOldPosts() {
    posts.update(posts => {
        const values = hashToArray(posts);
        return arrayToHash(values.slice(-MAX_POSTS));
    });
}
