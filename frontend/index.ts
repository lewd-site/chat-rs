import 'regenerator-runtime/runtime';

import * as Sentry from '@sentry/browser';
import AuthModal from './components/AuthModal.svelte';
import Gallery from './components/Gallery.svelte';
import MediaBox from './components/MediaBox.svelte';
import Menu from './components/Menu.svelte';
import PostForm from './components/PostForm.svelte';
import PostList from './components/PostList.svelte';
import PostPopups from './components/PostPopups.svelte';
import config from './config';
import EventEmitter from './event-emitter';
import { PostNotification } from './models/Notification';
import Api from './services/api';
import Coub from './services/coub';
import Sso from './services/sso';
import Tenor from './services/tenor';
import TikTok from './services/tiktok';
import YouTube from './services/youtube';
import { showAuthModal, userUuid, token } from './stores/auth';
import { nsfwMode, toggleNSFWMode } from './stores/files';
import Notifications from './stores/Notifications';
import NotificationPopups from './stores/NotificationPopups';
import { Posts, posts, addPosts, unloadOldPosts } from './stores/posts';
import Ws from './ws';
import utils from './utils';

import './styles/index.scss';
import 'prosemirror-view/style/prosemirror.css';
import '@simonwep/pickr/dist/themes/nano.min.css';

declare global {
  interface Window {
    api?: Api;
    coub?: Coub;
    eventBus?: EventEmitter;
    sso?: Sso;
    tenor?: Tenor;
    tiktok?: TikTok;
    ws?: Ws;
    youtube?: YouTube;
  }
}

if (typeof config.sentryDsn !== undefined && config.sentryDsn !== null) {
  try {
    Sentry.init({
      dsn: `${config.sentryDsn}`,
      release: process.env.GIT_SHA,
    });
  } catch (e) {
    console.error("Can't init Sentry: ", e);
  }
}

window.eventBus = new EventEmitter();

const components: {
  [key: string]: { new ({ target }: { target: Element }): unknown };
} = {
  '#auth-modal': AuthModal,
  '#gallery': Gallery,
  '#media-box': MediaBox,
  '#menu': Menu,
  '#post-form': PostForm,
  '#post-list': PostList,
  '#post-popups': PostPopups,
};

for (const key in components) {
  const container = document.querySelector(key);
  if (container === null) {
    throw new Error(`${key} not found`);
  }

  const Component = components[key];
  new Component({ target: container });
}

const authButton = document.getElementById('login');
authButton?.setAttribute('hidden', '');
authButton?.addEventListener('click', () => {
  showAuthModal.set(true);
});

window.api = new Api();
window.coub = new Coub();
window.sso = new Sso(() => window.api?.getToken());
window.tiktok = new TikTok();
window.tenor = new Tenor();
window.youtube = new YouTube();
window.ws = new Ws(config.wsUrl);

token.subscribe((token) => {
  if (!token) {
    return;
  }

  if (typeof config.sentryDsn !== undefined && config.sentryDsn !== null) {
    try {
      Sentry.setUser({
        id: token.user_uuid,
        username: token.user_name,
        email: token.user_email,
      });
    } catch (e) {
      console.warn("Can't set Sentry user context: ", e);
    }
  }
});

userUuid.subscribe((uuid) => {
  if (!uuid) {
    return;
  }

  window.api?.getNotifications().then((notifications) => {
    const notificationModels = notifications.map(PostNotification.createFromDTO);
    Notifications.replaceAll(notificationModels);
  });
});

let _posts: Posts = {};
posts.subscribe((posts) => (_posts = posts));

window.addEventListener('scroll', async () => {
  if (utils.isAtTop()) {
    const firstPost = Object.values(_posts)[0];
    if (!firstPost || +firstPost.id === 1) {
      return;
    }

    const oldPosts = await window.api?.getPostsBefore(firstPost.id);
    if (oldPosts) {
      addPosts(oldPosts);
      utils.maintainScrollBottom();
    }
  } else if (utils.isAtBottom()) {
    unloadOldPosts();
  }
});

document.getElementById('scroll-to-top')?.addEventListener('click', (e) => {
  e.preventDefault();
  utils.scrollToTop();
});

document.getElementById('scroll-to-bottom')?.addEventListener('click', (e) => {
  e.preventDefault();
  utils.scrollToBottom();
});

document.getElementById('toggle-nsfw')?.addEventListener('click', (e) => {
  e.preventDefault();
  toggleNSFWMode();
});

document.addEventListener('click', (e) => {
  if (!(e.target instanceof HTMLElement)) {
    return;
  }

  const target = e.target.closest('[data-ref-link]');
  if (target) {
    const attr = target.getAttribute('data-ref-link');
    const id = attr ? +attr : 0;
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

document.addEventListener('keydown', (e) => {
  if (
    !(e.target instanceof HTMLElement) ||
    e.target.tagName === 'INPUT' ||
    e.target.tagName === 'TEXTAREA'
  ) {
    return;
  }

  if (e.code === 'KeyB') {
    e.preventDefault();
    toggleNSFWMode();
  }
});

nsfwMode.subscribe((nsfwMode) => {
  if (nsfwMode) {
    document.body.classList.add('nsfw');
  } else {
    document.body.classList.remove('nsfw');
  }
});

setInterval(() => {
  if (!document.hidden) {
    NotificationPopups.handleTick();
  }
}, 1000);
