import 'regenerator-runtime/runtime';

import AuthModal from './components/AuthModal.svelte';
import MediaBox from './components/MediaBox.svelte';
import PostForm from './components/PostForm.svelte';
import PostList from './components/PostList.svelte';
import config from './config';
import Menu from './menu';
import Api from './services/api';
import Sso from './services/sso';
import * as stores from './stores';
import { Posts } from './stores';
import Ws from './ws';
import utils from './utils';

import './styles/index.scss';
import 'prosemirror-view/style/prosemirror.css';

declare global {
    interface Window {
        sso?: Sso;
        api?: Api;
        ws?: Ws;
    }
}

const authModalContainer = document.getElementById('auth-modal');
if (!authModalContainer) {
    throw new Error('#auth-modal not found');
}

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

const authModal = new AuthModal({ target: authModalContainer });
const postFrom = new PostForm({ target: postFormContainer });
const postList = new PostList({ target: postListContainer });
const mediaBox = new MediaBox({ target: mediaBoxContainer });

window.sso = new Sso();
window.api = new Api(window.sso);
window.ws = new Ws(config.wsUrl);

window.api.getLatestPosts().then(posts => {
    setTimeout(utils.scrollToBottom);
    stores.setPosts(posts);
});

const authButton = document.getElementById('login');
authButton?.setAttribute("hidden", "");
authButton?.addEventListener('click', e => {
    stores.authModal.set(true);
});

setTimeout(async () => {
    await window.sso!.get();
    if (window.sso!.hasAccessToken && !window.sso!.hasExpired) {
        authButton?.setAttribute('hidden', '');
    } else {
        authButton?.removeAttribute('hidden');
    }
}, 1000);

let _posts: Posts = {};
stores.posts.subscribe(posts => _posts = posts);

window.addEventListener('scroll', async () => {
    if (utils.isAtTop()) {
        const firstPost = Object.values(_posts)[0];
        if (!firstPost) {
            return;
        }

        const oldPosts = await window.api!.getPostsBefore(firstPost.id);
        stores.addPosts(oldPosts);
        utils.maintainScrollBottom();
    } else if (utils.isAtBottom()) {
        stores.unloadOldPosts();
    }
});

const menuElement = document.getElementById('menu');
if (menuElement) {
    const menu = new Menu(menuElement);
}
