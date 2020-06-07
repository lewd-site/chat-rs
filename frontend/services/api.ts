import axios from 'axios';

import { Post } from '../types';

interface SubmitPostRequest {
    readonly name: string;
    readonly message: string;
}

interface PostListResponse {
    readonly items: Post[];
}

interface PostResponse {
    readonly item: Post;
}

export class Api {
    public static async submitPost(data: SubmitPostRequest): Promise<Post> {
        const response = await axios.post<PostResponse>('/api/v1/posts', data);
        return response.data.item;
    }

    public static async getLatestPosts(): Promise<Post[]> {
        const response = await axios.get<PostListResponse>('/api/v1/posts');
        return response.data.items;
    }

    public static async getPost(postId: number): Promise<Post> {
        const response = await axios.get<PostResponse>(`/api/v1/posts/${postId}`);
        return response.data.item;
    }
}

export default Api;
