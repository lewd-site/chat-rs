import { Embed } from '../types';

const POPUP_HEADER_PADDIGN = 24;

export async function getEmbed(url: string): Promise<Embed> {
    if (/^(?:https?:\/\/)?(?:www\.)?coub\.com\/view\//i.test(url)) {
        const _url = encodeURIComponent(url.replace(/^https?:\/\//, ''));
        const data = await window.coub!.getCoubInfo(_url);

        const html = data.html.replace('muted=true', 'muted=false')
            .replace(/width="\d+"/i, 'width="100%"')
            .replace(/height="\d+"/i, 'height="100%"');

        const embed: Embed = {
            id: _url,
            name: data.title,
            mimetype: 'video/x-coub',
            thumbnail_width: +data.thumbnail_width,
            thumbnail_height: +data.thumbnail_height,
            thumbnail_url: data.thumbnail_url,
            width: +data.width,
            height: +data.height + POPUP_HEADER_PADDIGN,
            html,
        };
        return embed;
    } else if (/^(?:https?:\/\/)?(?:www\.)?(tiktok\.com)\/@([0-9a-z_-]+)\/video\/(\d+)/i.test(url)) {
        const matches = url.match(/^(?:https?:\/\/)?(?:www\.)?(?:tiktok\.com)\/@([0-9a-z_-]+)\/video\/(\d+)/i);
        const normalizedUrl = `https://www.tiktok.com/@${matches![1]}/video/${matches![2]}`;
        const videoId = encodeURIComponent(normalizedUrl);
        const data = await window.tiktok!.getVideoInfo(videoId);

        const html = data.html.replace('<script async src="https://www.tiktok.com/embed.js"></script>', '');

        const embed: Embed = {
            id: url,
            name: data.title,
            mimetype: 'video/x-tiktok',
            thumbnail_width: +data.thumbnail_width,
            thumbnail_height: +data.thumbnail_height,
            thumbnail_url: data.thumbnail_url,
            width: +data.thumbnail_width,
            height: +data.thumbnail_height + POPUP_HEADER_PADDIGN,
            min_width: 325,
            max_width: 605,
            html: html,
        };

        return embed;
    } else if (/^(?:https?:\/\/)?(?:www\.)?(?:youtube\.com\/(?:watch|embed|v)|youtu\.be\/)/i.test(url)) {
        const _url = encodeURIComponent(url.replace(/^https?:\/\//, ''));
        const data = await window.youtube!.getVideoInfo(_url);

        const html = data.html.replace(/src="([^"]+)"/i, 'src="$1&autoplay=1"')
            .replace(/width="\d+"/i, 'width="100%"')
            .replace(/height="\d+"/i, 'height="100%"');

        const embed: Embed = {
            id: _url,
            name: data.title,
            mimetype: 'video/x-youtube',
            thumbnail_width: +data.thumbnail_width,
            thumbnail_height: +data.thumbnail_height,
            thumbnail_url: data.thumbnail_url,
            width: +data.width,
            height: +data.height + POPUP_HEADER_PADDIGN,
            min_width: 320,
            html,
        };

        return embed;
    } else {
        throw new Error("Unknown embed type");
    }
}
