import axios from 'axios';
import config from '../config';

const PER_PAGE = 40;

interface Media {
  readonly url: string;
  readonly size: number;
  readonly dims: number[];
}

interface Gif {
  readonly id: string;
  readonly url: string;
  readonly title: string;
  readonly media: { [key: string]: Media }[];
}

export interface TenorSearchResponse {
  readonly results: Gif[];
}

export interface TenorShareResponse {
  readonly status: string;
}

export interface TenorAnonIdResponse {
  readonly anon_id: string;
}

export class Tenor {
  public async createAnonId(): Promise<string> {
    const url = `https://api.tenor.com/v1/anonid?key=${config.tenorKey}`;
    const response = await axios.get<TenorAnonIdResponse>(url);
    return response.data.anon_id;
  }

  public async getAnonId(): Promise<string> {
    if (localStorage.getItem('tenor_aid')) {
      return localStorage['tenor_aid'];
    }

    return localStorage['tenor_aid'] = await this.createAnonId();
  }

  public async search(query: string, page = 0): Promise<TenorSearchResponse> {
    const q = encodeURIComponent(query);
    const limit = PER_PAGE;
    const pos = PER_PAGE * page;
    const anon_id = await this.getAnonId();
    const url = `https://api.tenor.com/v1/search?key=${config.tenorKey}&q=${q}&limit=${limit}&pos=${pos}&anon_id=${anon_id}&` +
      `locale=ru_RU&contentfilter=off&media_filter=basic&ar_range=all`;
    const response = await axios.get<TenorSearchResponse>(url);
    return response.data;
  }

  public async share(query: string, id: string): Promise<TenorShareResponse> {
    const q = encodeURIComponent(query);
    const anon_id = await this.getAnonId();
    const url = `https://api.tenor.com/v1/registershare?key=${config.tenorKey}&q=${q}&id=${id}&anon_id=${anon_id}&locale=ru_RU`;
    const response = await axios.get<TenorShareResponse>(url);
    return response.data;
  }
}

export default Tenor;
