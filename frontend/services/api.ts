import axios from 'axios';

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
    public static async submitPost(data: SubmitPostRequest): Promise<Post> {
        const formData = new FormData();
        formData.append('name', data.name);
        formData.append('message', data.message);

        if (data.files && data.files.length) {
            [...data.files].forEach(file => {
                formData.append('file', file, file.name);
            });
        }

        const config = {
            headers: { 'Content-Type': 'multipart/form-data' },
        };

        const response = await axios.post<PostResponse>('/api/v1/posts', formData, config);
        return response.data.item;
    }

    public static async getLatestPosts(): Promise<Post[]> {
        const response = await axios.get<PostListResponse>('/api/v1/posts');
        return response.data.items;
    }

    public static async getPostsBefore(postId: number): Promise<Post[]> {
        const response = await axios.get<PostListResponse>(`/api/v1/posts?before_id=${postId}`);
        return response.data.items;
    }

    public static async getPost(postId: number): Promise<Post> {
        const response = await axios.get<PostResponse>(`/api/v1/posts/${postId}`);
        return response.data.item;
    }
}

export default Api;
