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
import Sso from './services/sso';
import { showAuthModal, token, userUuid } from './stores/auth';
import { setNotifications } from './stores/notifications';
import { nsfwMode, toggleNSFWMode } from './stores/files';
import { Posts, posts, setPosts, addPosts, unloadOldPosts } from './stores/posts';
import Ws from './ws';
import utils from './utils';

import './styles/index.scss';
import 'prosemirror-view/style/prosemirror.css';

declare global {
    interface Window {
        readonly Sentry?: any;

        sso?: Sso;
        api?: Api;
        ws?: Ws;
        eventBus?: EventEmitter;
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

window.sso = new Sso();
window.api = new Api(window.sso);
window.ws = new Ws(config.wsUrl);

window.api.getLatestPosts().then(posts => {
    setTimeout(utils.scrollToBottom);
    setPosts(posts);
});

const authButton = document.getElementById('login');
authButton?.setAttribute('hidden', '');
authButton?.addEventListener('click', e => {
    showAuthModal.set(true);
});

userUuid.subscribe(uuid => {
    if (!uuid) {
        return;
    }

    window.api?.getNotifications().then(notifications => setNotifications(notifications));
});

setTimeout(async () => {
    token.set(await window.sso!.get());

    if (!window.sso!.hasAccessToken || window.sso!.hasExpired) {
        const email = localStorage['auth_email'];
        if (window.sso!.hasRefreshToken && email) {
            try {
                token.set(await window.sso!.refreshByEmail(email));
            } catch (e) { }
        }
    }

    if (window.sso!.hasAccessToken && !window.sso!.hasExpired) {
        authButton?.setAttribute('hidden', '');
    } else {
        authButton?.removeAttribute('hidden');
    }
}, 1000);

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
