import { createPcg32, randomInt } from 'pcg';
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
  posts.update((posts) => {
    const post = posts[id];
    if (typeof post !== 'undefined') {
      return { ...posts, [id]: callback(post) };
    } else {
      return posts;
    }
  });
}

const postRand: { [id: number]: unknown } = {};

const VOCAROO_PATTERN = /^(?:https?:\/\/)?(?:www\.)?(?:voca\.ro|vocaroo\.com)\/([0-9a-z_-]+)$/i;
const SPOTIFY_PATTERN = /^(?:https?:\/\/)?open.spotify.com\/(album|artist|playlist|track)\/([0-9a-z_-]+)/i;
const COUB_PATTERN = /^(?:https?:\/\/)?(?:www\.)?coub\.com\/view\//i;
const TIKTOK_PATTERN = /^(?:https?:\/\/)?(?:www\.)?(?:tiktok\.com)\/@([0-9a-z_-]+)\/video\/(\d+)/i;
const TIKTOK_SHORT_PATTERN = /^(?:https?:\/\/)?(?:vm\.tiktok\.com)\/([0-9a-z_-]+)/i;
const YOUTUBE_PATTERN = /^(?:https?:\/\/)?(?:www\.)?(?:youtube\.com\/(?:watch|embed|v)|youtu\.be\/)/i;

function processMarkup(markup: Markup, post: Post): Markup {
  if (markup.type === 'Text') {
    return markup;
  } else if (markup.type === 'Tag') {
    let { children } = markup;
    children = children.map((m) => processMarkup(m, post));

    const { tag } = markup;
    if (tag.type === 'RefLink') {
      setTimeout(() =>
        updatePost(+tag.id, (targetPost) => {
          const reply_from =
            typeof targetPost.reply_from !== 'undefined'
              ? Array.from(new Set([...targetPost.reply_from, post.id]))
              : [post.id];
          return { ...targetPost, reply_from };
        }),
      );
    } else if (tag.type === 'Link') {
      if (VOCAROO_PATTERN.test(tag.url)) {
        const matches = tag.url.match(VOCAROO_PATTERN);
        if (matches) {
          const url = `https://vocaroo.com/embed/${matches[1]}`;
          const html = `<iframe class="markup_vocaroo" width="300" height="60" frameborder="0" src="${url}"></iframe>`;

          return {
            type: 'Tag',
            tag: { type: 'HTML', content: html },
            children: [],
          };
        }
      } else if (SPOTIFY_PATTERN.test(tag.url)) {
        const matches = tag.url.match(SPOTIFY_PATTERN);
        if (matches) {
          const url = `https://open.spotify.com/embed/${matches[1]}/${matches[2]}`;
          const html = `<iframe class="markup_spotify" width="300" height="300" allowtransparency="true" frameborder="0" allow="encrypted-media" src="${url}"></iframe>`;

          return {
            type: 'Tag',
            tag: { type: 'HTML', content: html },
            children: [],
          };
        }
      } else if (
        COUB_PATTERN.test(tag.url) ||
        TIKTOK_PATTERN.test(tag.url) ||
        TIKTOK_SHORT_PATTERN.test(tag.url) ||
        YOUTUBE_PATTERN.test(tag.url)
      ) {
        if (post.embeds === undefined) {
          post.embeds = [tag.url];
        } else {
          post.embeds.push(tag.url);
        }
      }
    } else if (tag.type === 'Dice') {
      const count = Math.max(1, Math.min(20, tag.count));
      const max = Math.max(1, Math.min(1000000000, tag.max));
      const results: number[] = [];

      if (typeof postRand[post.id] === 'undefined') {
        postRand[post.id] = createPcg32({}, post.id ^ Date.parse(post.created_at), 1);
      }

      for (let i = 0; i < count; ++i) {
        const [value, rand] = randomInt(0, max + 1, postRand[post.id]);
        results.push(value);
        postRand[post.id] = rand;
      }

      const sum = results.reduce((acc, cur) => acc + cur, 0);
      const avg = sum / results.length;

      const content = `<span class="markup_dice">##${count}d${max}## = ${results.join(
        ', ',
      )} (sum ${sum}, avg ${avg})</span>`;
      return { type: 'Tag', tag: { type: 'HTML', content }, children: [] };
    }

    return { ...markup, children };
  } else {
    return markup;
  }
}

function processPost(post: Post): Post {
  const message = post.message.map((m) => processMarkup(m, post));
  return { ...post, message };
}

export function addPost(post: Post): void {
  posts.update((posts) => {
    return { ...posts, [post.id]: processPost(post) };
  });
}

export function addPosts(newPosts: Post[]): void {
  posts.update((posts) => {
    const result = { ...posts };
    newPosts.forEach((p) => (result[p.id] = processPost(p)));
    return result;
  });
}

export function setPosts(newPosts: Post[]): void {
  posts.set({});
  addPosts(newPosts);
}

export function unloadOldPosts(): void {
  posts.update((posts) => {
    const values = hashToArray(posts);
    return arrayToHash(values.slice(-MAX_POSTS));
  });
}

export const visiblePosts = writable<number[]>([]);
