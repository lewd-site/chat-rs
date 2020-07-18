import axios from 'axios';
import config from '../config';

export interface CoubOEmbedResponse {
  readonly type: string;
  readonly version: string;
  readonly title: string;
  readonly html: string;
  readonly author_name: string;
  readonly channel_url: string;
  readonly provider_name: string;
  readonly provider_url: string;
  readonly thumbnail_width: string;
  readonly thumbnail_height: string;
  readonly thumbnail_url: string;
  readonly width: string;
  readonly height: string;
}

export class Coub {
  private readonly cache: { [coubId: string]: CoubOEmbedResponse } = {};

  public async getCoubInfo(coubId: string): Promise<CoubOEmbedResponse> {
    if (typeof this.cache[coubId] !== 'undefined') {
      return this.cache[coubId];
    }

    const response = await axios.get<CoubOEmbedResponse>(`${config.coubProxyUrl}${coubId}`);
    return (this.cache[coubId] = response.data);
  }
}

export default Coub;
