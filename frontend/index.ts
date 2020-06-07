import 'regenerator-runtime/runtime';

import PostForm from './components/PostForm.svelte';
import PostList from './components/PostList.svelte';
import config from './config';
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

const postFrom = new PostForm({ target: postFormContainer });
const postList = new PostList({ target: postListContainer });

posts.subscribe(() => {
    setTimeout(() => {
        const scrollingElement = (document.scrollingElement || document.body);
        scrollingElement.scrollTop = scrollingElement.scrollHeight;
    });
});

Api.getLatestPosts().then(addPosts);

const ws = new Ws(config.wsUrl);
