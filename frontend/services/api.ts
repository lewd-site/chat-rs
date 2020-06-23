import axios, { AxiosRequestConfig } from 'axios';

import Sso from './sso';
import { token } from '../stores/auth';
import { Post, Notification } from '../types';

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

interface NotificationResponse {
    readonly item: Notification;
}

interface NotificationListResponse {
    readonly items: Notification[];
}

export class Api {
    public constructor(private readonly sso: Sso) { }

    private getToken = async (): Promise<string | null> => {
        token.set(await this.sso.get());

        if (this.sso.hasAccessToken && !this.sso.hasExpired) {
            return this.sso.accessToken;
        }

        if (this.sso.hasRefreshToken) {
            const email = localStorage['auth_email'];
            if (email) {
                try {
                    token.set(await this.sso.refreshByEmail(email));

                    return this.sso.accessToken;
                } catch (e) { }
            }
        }

        return null;
    };

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

        const token = await this.getToken();
        if (token) {
            config.headers['Authorization'] = `Bearer ${token}`;
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

    public getNotifications = async (): Promise<Notification[]> => {
        const config: AxiosRequestConfig = { headers: {} };

        const token = await this.getToken();
        if (token) {
            config.headers['Authorization'] = `Bearer ${token}`;
        }

        const response = await axios.get<NotificationListResponse>('/api/v1/notifications', config);
        return response.data.items;
    };

    public readNotification = async (id: number): Promise<Notification> => {
        const config: AxiosRequestConfig = { headers: {} };

        const token = await this.getToken();
        if (token) {
            config.headers['Authorization'] = `Bearer ${token}`;
        }

        const response = await axios.post<NotificationResponse>(`/api/v1/notifications/${id}/read`, {}, config);
        return response.data.item;
    };
}

export default Api;
