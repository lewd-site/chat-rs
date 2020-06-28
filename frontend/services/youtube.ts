import axios from 'axios';

const youtubeProxyUrl = '/api/v1/oembed/youtube/';

export interface YouTubeOEmbedResponse {
    readonly type: string;
    readonly version: string;
    readonly title: string;
    readonly html: string;
    readonly author_name: string;
    readonly author_url: string;
    readonly provider_name: string;
    readonly provider_url: string;
    readonly thumbnail_width: number;
    readonly thumbnail_height: number;
    readonly thumbnail_url: string;
    readonly width: number;
    readonly height: number;
}

export class YouTube {
    private readonly cache: { [videoId: string]: YouTubeOEmbedResponse } = {};

    public async getVideoInfo(videoId: string): Promise<YouTubeOEmbedResponse> {
        if (typeof this.cache[videoId] !== 'undefined') {
            return this.cache[videoId];
        }

        const response = await axios.get<YouTubeOEmbedResponse>(`${youtubeProxyUrl}${videoId}`);
        return this.cache[videoId] = response.data;
    }
}

export default YouTube;
