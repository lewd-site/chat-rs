import 'regenerator-runtime/runtime';

import AuthModal from './components/AuthModal.svelte';
import MediaBox from './components/MediaBox.svelte';
import Menu from './components/Menu.svelte';
import PostForm from './components/PostForm.svelte';
import PostList from './components/PostList.svelte';
import PostPopups from './components/PostPopups.svelte';
import config from './config';
import EventEmitter from './event-emitter';
import Api from './services/api';
import Coub from './services/coub';
import Sso from './services/sso';
import YouTube from './services/youtube';
import { showAuthModal, userUuid } from './stores/auth';
import { setNotifications } from './stores/notifications';
import { nsfwMode, toggleNSFWMode } from './stores/files';
import { Posts, posts, addPosts, unloadOldPosts } from './stores/posts';
import Ws from './ws';
import utils from './utils';

import './styles/index.scss';
import 'prosemirror-view/style/prosemirror.css';
import '@simonwep/pickr/dist/themes/nano.min.css';

declare global {
    interface Window {
        readonly Sentry?: any;

        api?: Api;
        coub?: Coub;
        eventBus?: EventEmitter;
        sso?: Sso;
        ws?: Ws;
        youtube?: YouTube;
    }
}

if (typeof config.sentryDsn !== 'undefined') {
    try {
        window.Sentry.init({ dsn: config.sentryDsn });
    } catch (e) {
        console.log("Can't init Sentry: ", e);
    }
}

window.eventBus = new EventEmitter();

const menuContainer = document.getElementById('menu');
if (!menuContainer) {
    throw new Error('#menu not found');
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

const postPopupsContainer = document.getElementById('post-popups');
if (!postPopupsContainer) {
    throw new Error('#post-popups not found');
}

const menu = new Menu({ target: menuContainer });
const authModal = new AuthModal({ target: authModalContainer });
const postFrom = new PostForm({ target: postFormContainer });
const postList = new PostList({ target: postListContainer });
const mediaBox = new MediaBox({ target: mediaBoxContainer });
const postpopUps = new PostPopups({ target: postPopupsContainer });

const authButton = document.getElementById('login');
authButton?.setAttribute('hidden', '');
authButton?.addEventListener('click', e => {
    showAuthModal.set(true);
});

window.api = new Api();
window.coub = new Coub();
window.sso = new Sso(() => window.api!.getToken());
window.ws = new Ws(config.wsUrl);
window.youtube = new YouTube();

userUuid.subscribe(uuid => {
    if (!uuid) {
        return;
    }

    window.api?.getNotifications().then(notifications => {
        setNotifications(notifications);
    });
});

let _posts: Posts = {};
posts.subscribe(posts => _posts = posts);

window.addEventListener('scroll', async () => {
    if (utils.isAtTop()) {
        const firstPost = Object.values(_posts)[0];
        if (!firstPost || +firstPost.id === 1) {
            return;
        }

        const oldPosts = await window.api!.getPostsBefore(firstPost.id);
        addPosts(oldPosts);
        utils.maintainScrollBottom();
    } else if (utils.isAtBottom()) {
        unloadOldPosts();
    }
});

document.getElementById('scroll-to-top')?.addEventListener('click', e => {
    e.preventDefault();
    utils.scrollToTop();
});

document.getElementById('scroll-to-bottom')?.addEventListener('click', e => {
    e.preventDefault();
    utils.scrollToBottom();
});

document.getElementById('toggle-nsfw')?.addEventListener('click', e => {
    e.preventDefault();
    toggleNSFWMode();
});

document.addEventListener('click', e => {
    if (!(e.target instanceof HTMLElement)) {
        return;
    }

    const target = e.target.closest('[data-ref-link]');
    if (target) {
        const id = +target.getAttribute('data-ref-link')!;
        const post = document.getElementById(`post_${id}`);
        if (post) {
            e.preventDefault();
            utils.scrollToElement(post);
            post.classList.add('post_highlight');
            setTimeout(() => post.classList.remove('post_highlight'), 500);
            return false;
        }
    }
});

document.addEventListener('keydown', e => {
    if (!(e.target instanceof HTMLElement) ||
        e.target.tagName === 'INPUT' ||
        e.target.tagName === 'TEXTAREA') {
        return;
    }

    if (e.code === 'KeyB') {
        e.preventDefault();
        toggleNSFWMode();
    }
});

nsfwMode.subscribe(nsfwMode => {
    if (nsfwMode) {
        document.body.classList.add('nsfw');
    } else {
        document.body.classList.remove('nsfw');
    }
});
