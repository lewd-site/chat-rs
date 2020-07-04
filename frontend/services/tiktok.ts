import axios from 'axios';

export interface TikTokOEmbedResponse {
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
    readonly width: string;
    readonly height: string;
}

export class TikTok {
    private readonly cache: { [videoId: string]: TikTokOEmbedResponse } = {};

    public async getVideoInfo(videoId: string): Promise<TikTokOEmbedResponse> {
        if (typeof this.cache[videoId] !== 'undefined') {
            return this.cache[videoId];
        }

        const response = await axios.get<TikTokOEmbedResponse>(`https://www.tiktok.com/oembed?url=${videoId}`);
        return this.cache[videoId] = response.data;
    }
}

export default TikTok;
