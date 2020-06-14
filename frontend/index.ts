import 'regenerator-runtime/runtime';

import MediaBox from './components/MediaBox.svelte';
import PostForm from './components/PostForm.svelte';
import PostList from './components/PostList.svelte';
import config from './config';
import Menu from './menu';
import Api from './services/api';
import { setPosts, posts, Posts, addPosts, unloadOldPosts } from './stores';
import Ws from './ws';

import './styles/index.scss';
import utils from './utils';

const postFormContainer = document.getElementById('post-form');
if (!postFormContainer) {
    throw new Error('#post-form not found');
}

const postListContainer = document.getElementById('post-list');
if (!postListContainer) {
    throw new Error('#post-list not found');
}

const mediaBoxContainer = document.getElementById('media-box');
if (!mediaBoxContainer) {
    throw new Error('#media-box not found');
}

const postFrom = new PostForm({ target: postFormContainer });
const postList = new PostList({ target: postListContainer });
const mediaBox = new MediaBox({ target: mediaBoxContainer });

let _posts: Posts = {};
posts.subscribe(posts => _posts = posts);

window.addEventListener('scroll', async () => {
    if (utils.isAtTop()) {
        const firstPost = Object.values(_posts)[0];
        if (!firstPost) {
            return;
        }

        const oldPosts = await Api.getPostsBefore(firstPost.id);
        addPosts(oldPosts);
        utils.maintainScrollBottom();
    } else if (utils.isAtBottom()) {
        unloadOldPosts();
    }
});

Api.getLatestPosts().then(posts => {
    setTimeout(utils.scrollToBottom);
    setPosts(posts);
});

const ws = new Ws(config.wsUrl);

const menuElement = document.getElementById('menu');
if (menuElement) {
    const menu = new Menu(menuElement);
}
