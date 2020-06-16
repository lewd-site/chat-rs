import axios, { AxiosRequestConfig } from 'axios';

import Sso from './sso';
import { Post } from '../types';

interface SubmitPostRequest {
    readonly name: string;
    readonly message: string;
    readonly files: FileList | File[];
}

interface PostListResponse {
    readonly items: Post[];
}

interface PostResponse {
    readonly item: Post;
}

export class Api {
    public constructor(private readonly sso: Sso) { }

    public submitPost = async (data: SubmitPostRequest): Promise<Post> => {
        const formData = new FormData();
        formData.append('name', data.name);
        formData.append('message', data.message);

        if (data.files && data.files.length) {
            [...data.files].forEach(file => {
                formData.append('file', file, file.name);
            });
        }

        const config: AxiosRequestConfig = {
            headers: { 'Content-Type': 'multipart/form-data' },
        };

        await this.sso.get();
        if (this.sso.hasAccessToken && !this.sso.hasExpired) {
            config.headers['Authorization'] = `Bearer ${this.sso.accessToken}`;
        } else if (this.sso.hasRefreshToken) {
            const email = localStorage['auth_email'];
            if (email) {
                try {
                    await this.sso.refreshByEmail(email);
                    config.headers['Authorization'] = `Bearer ${this.sso.accessToken}`;
                } catch (e) { }
            }
        }

        const response = await axios.post<PostResponse>('/api/v1/posts', formData, config);
        return response.data.item;
    };

    public getLatestPosts = async (): Promise<Post[]> => {
        const response = await axios.get<PostListResponse>('/api/v1/posts');
        return response.data.items;
    };

    public getPostsBefore = async (postId: number): Promise<Post[]> => {
        const response = await axios.get<PostListResponse>(`/api/v1/posts?before_id=${postId}`);
        return response.data.items;
    };

    public getPost = async (postId: number): Promise<Post> => {
        const response = await axios.get<PostResponse>(`/api/v1/posts/${postId}`);
        return response.data.item;
    };
}

export default Api;
