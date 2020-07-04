import { writable } from 'svelte/store';

import { RefLink, Post, Link, Embed } from '../types';

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

function updatePost(post: Post) {
    posts.update(posts => {
        return { ...posts, [post.id]: post };
    });
}

function processPost(post: Post, allPosts: Post[]) {
    post.message.forEach((segment, segmentIndex) => {
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

        const linkTags = segment.tags.filter(tag => tag.type === 'Link') as Link[];
        linkTags.forEach(async (tag, tagIndex) => {
            if (/^(?:https?:\/\/)?(?:www\.)?(?:voca\.ro|vocaroo\.com)\/([A-Za-z0-9]+)$/.test(tag.url)) {
                const matches = tag.url.match(/^(?:https?:\/\/)?(?:www\.)?(?:voca\.ro|vocaroo\.com)\/([A-Za-z0-9]+)$/);
                const html = `<iframe class="markup_vocaroo" width="300" height="60" src="https://vocaroo.com/embed/${matches![1]}" frameborder="0"></iframe>`;
                post.message[segmentIndex] = { tags: [{ type: 'HTML', content: html }], text: '' };

                updatePost(post);
            } else if (/^(?:https?:\/\/)?(?:www\.)?coub\.com\/view\//i.test(tag.url)) {
                const url = encodeURIComponent(tag.url.replace(/^https?:\/\//, ''));
                const data = await window.coub!.getCoubInfo(url);

                const text = data.title;
                const tags = [...segment.tags];
                tags.splice(tagIndex, 1, { ...tag, icon: 'coub' });

                post.message[segmentIndex] = { ...post.message[segmentIndex], text, tags };

                const html = data.html.replace('muted=true', 'muted=false')
                    .replace(/width="\d+"/i, 'width="100%"')
                    .replace(/height="\d+"/i, 'height="100%"');

                const embed: Embed = {
                    id: tag.url,
                    name: data.title,
                    mimetype: 'video/x-coub',
                    thumbnail_width: +data.thumbnail_width,
                    thumbnail_height: +data.thumbnail_height,
                    thumbnail_url: data.thumbnail_url,
                    width: +data.width,
                    height: +data.height + POPUP_HEADER_PADDIGN,
                    html,
                };

                if (post.embeds === undefined) {
                    post.embeds = [embed];
                } else {
                    post.embeds.push(embed);
                }

                updatePost(post);
            } else if (/^(?:https?:\/\/)?(?:www\.)?(tiktok\.com)\/@([0-9a-z_-]+)\/video\/(\d+)/i.test(tag.url)) {
                const matches = tag.url.match(/^(?:https?:\/\/)?(?:www\.)?(?:tiktok\.com)\/@([0-9a-z_-]+)\/video\/(\d+)/i);
                const normalizedUrl = `https://www.tiktok.com/@${matches![1]}/video/${matches![2]}`;
                const videoId = encodeURIComponent(normalizedUrl);
                const data = await window.tiktok!.getVideoInfo(videoId);

                const text = data.title;
                const tags = [...segment.tags];
                tags.splice(tagIndex, 1, { ...tag, icon: 'tiktok' });

                post.message[segmentIndex] = { ...post.message[segmentIndex], text, tags };

                const html = data.html.replace('<script async src="https://www.tiktok.com/embed.js"></script>', '');

                const embed: Embed = {
                    id: tag.url,
                    name: data.title,
                    mimetype: 'video/x-tiktok',
                    thumbnail_width: +data.thumbnail_width,
                    thumbnail_height: +data.thumbnail_height,
                    thumbnail_url: data.thumbnail_url,
                    width: +data.thumbnail_width,
                    height: +data.thumbnail_height + POPUP_HEADER_PADDIGN,
                    min_width: 325,
                    max_width: 605,
                    html: html,
                };

                if (post.embeds === undefined) {
                    post.embeds = [embed];
                } else {
                    post.embeds.push(embed);
                }

                updatePost(post);
            } else if (/^(?:https?:\/\/)?(?:www\.)?(?:youtube\.com\/(?:watch|embed|v)|youtu\.be\/)/i.test(tag.url)) {
                const url = encodeURIComponent(tag.url.replace(/^https?:\/\//, ''));
                const data = await window.youtube!.getVideoInfo(url);

                const text = data.title;
                const tags = [...segment.tags];
                tags.splice(tagIndex, 1, { ...tag, icon: 'youtube' });

                post.message[segmentIndex] = { ...post.message[segmentIndex], text, tags };

                const html = data.html.replace(/src="([^"]+)"/i, 'src="$1&autoplay=1"')
                    .replace(/width="\d+"/i, 'width="100%"')
                    .replace(/height="\d+"/i, 'height="100%"');

                const embed: Embed = {
                    id: tag.url,
                    name: data.title,
                    mimetype: 'video/x-youtube',
                    thumbnail_width: +data.thumbnail_width,
                    thumbnail_height: +data.thumbnail_height,
                    thumbnail_url: data.thumbnail_url,
                    width: +data.width,
                    height: +data.height + POPUP_HEADER_PADDIGN,
                    min_width: 320,
                    html,
                };

                if (post.embeds === undefined) {
                    post.embeds = [embed];
                } else {
                    post.embeds.push(embed);
                }

                updatePost(post);
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
