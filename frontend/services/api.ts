import axios, { AxiosRequestConfig } from 'axios';

import { token } from '../stores/auth';
import { Post, Notification, PostNotification } from '../types';

interface SubmitPostRequest {
  readonly name: string;
  readonly tripcode: string;
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
  readonly item: PostNotification;
}

interface NotificationListResponse {
  readonly items: PostNotification[];
}

interface FileResponse {
  readonly item: File;
}

interface FileListResponse {
  readonly items: File[];
}

export class Api {
  public getToken = async (): Promise<string | null> => {
    const authButton = document.getElementById('login');
    try {
      await window.sso?.get();
      if (window.sso?.hasAccessToken && !window.sso?.hasExpired) {
        token.set(window.sso?.accessTokenData);
        authButton?.setAttribute('hidden', '');
      } else {
        const email = localStorage['auth_email'];
        if (window.sso?.hasRefreshToken && email) {
          await window.sso?.refreshByEmail(email);
          token.set(window.sso?.accessTokenData);
          authButton?.setAttribute('hidden', '');
        } else {
          token.set(null);
          authButton?.removeAttribute('hidden');
        }
      }
    } catch (e) {
      token.set(null);
      authButton?.removeAttribute('hidden');
    }

    return typeof window.sso !== 'undefined' ? window.sso.accessToken : null;
  };

  public submitPost = async (data: SubmitPostRequest): Promise<Post> => {
    const formData = new FormData();
    formData.append('name', data.name);
    formData.append('tripcode', data.tripcode);
    formData.append('message', data.message);

    if (data.files && data.files.length) {
      [...data.files].forEach((file) => {
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

  public getNotifications = async (): Promise<PostNotification[]> => {
    const config: AxiosRequestConfig = { headers: {} };

    const token = await this.getToken();
    if (window.sso?.hasAccessToken && !window.sso?.hasExpired) {
      config.headers['Authorization'] = `Bearer ${token}`;
    } else {
      return [];
    }

    const response = await axios.get<NotificationListResponse>('/api/v1/notifications', config);
    return response.data.items.map((item) => ({ ...item, type: 'post' }));
  };

  public readNotification = async (id: number): Promise<PostNotification> => {
    const config: AxiosRequestConfig = { headers: {} };

    const token = await this.getToken();
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await axios.post<NotificationResponse>(
      `/api/v1/notifications/${id}/read`,
      {},
      config,
    );
    return { ...response.data.item, type: 'post' };
  };

  public deleteNotification = async (id: number): Promise<Notification> => {
    const config: AxiosRequestConfig = { headers: {} };

    const token = await this.getToken();
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await axios.delete<NotificationResponse>(
      `/api/v1/notifications/${id}`,
      config,
    );
    return { ...response.data.item, type: 'post' };
  };

  public getLatestFiles = async (): Promise<File[]> => {
    const config: AxiosRequestConfig = { headers: {} };

    const token = await this.getToken();
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    } else {
      return [];
    }

    const response = await axios.get<FileListResponse>('/api/v1/files', config);
    return response.data.items;
  };

  public createFavoriteFile = async (md5: string): Promise<File> => {
    const request = { md5 };
    const config: AxiosRequestConfig = { headers: {} };

    const token = await this.getToken();
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await axios.post<FileResponse>('/api/v1/favorites', request, config);
    return response.data.item;
  };

  public deleteFavoriteFile = async (md5: string): Promise<File> => {
    const config: AxiosRequestConfig = { headers: {} };

    const token = await this.getToken();
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await axios.delete<FileResponse>(`/api/v1/favorites/${md5}`, config);
    return response.data.item;
  };

  public getFavoriteFiles = async (): Promise<File[]> => {
    const config: AxiosRequestConfig = { headers: {} };

    const token = await this.getToken();
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    } else {
      return [];
    }

    const response = await axios.get<FileListResponse>('/api/v1/favorites', config);
    return response.data.items;
  };
}

export default Api;
