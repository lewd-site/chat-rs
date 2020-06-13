import 'regenerator-runtime/runtime';

import MediaBox from './components/MediaBox.svelte';
import PostForm from './components/PostForm.svelte';
import PostList from './components/PostList.svelte';
import config from './config';
import Menu from './menu';
import Api from './services/api';
import { addPosts, posts } from './stores';
import Ws from './ws';

import './styles/index.scss';

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

posts.subscribe(() => {
    const scrollingElement = (document.scrollingElement || document.body);
    const useAutoscroll = (scrollingElement as any).offsetHeight + scrollingElement.scrollTop > scrollingElement.scrollHeight - 20;
    if (useAutoscroll) {
        setTimeout(() => {
            scrollingElement.scrollTop = scrollingElement.scrollHeight;
        });
    }
});

Api.getLatestPosts().then(addPosts);

const ws = new Ws(config.wsUrl);

const menuElement = document.getElementById('menu');
if (menuElement) {
    const menu = new Menu(menuElement);
}
