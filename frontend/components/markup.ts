import { formatName } from './post';
import { posts, Posts } from '../stores/posts';
import { Markup } from '../types';

let _posts: Posts = {};
posts.subscribe(posts => _posts = posts);

function escapeHtml(html: string): string {
    return html
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#039;');
}

export function markup(m: Markup | Markup[]): string {
    if (Array.isArray(m)) {
        return m.map(markup).join('');
    }

    let html = escapeHtml(m.text);
    const tags = [...m.tags];
    tags.reverse();
    tags.forEach(tag => {
        switch (tag.type) {
            case 'Bold':
                html = `<strong class="markup markup_bold">${html}</strong>`;
                break;

            case 'Italic':
                html = `<em class="markup markup_italic">${html}</em>`;
                break;

            case 'Underline':
                html = `<span class="markup markup_underline">${html}</span>`;
                break;

            case 'Strike':
                html = `<del class="markup markup_strike">${html}</del>`;
                break;

            case 'Superscript':
                html = `<sup class="markup markup_superscript">${html}</sup>`;
                break;

            case 'Subscript':
                html = `<sub class="markup markup_subscript">${html}</sub>`;
                break;

            case 'Code':
                html = `<pre class="markup markup_code">${html}</pre>`;
                break;

            case 'CodeBlock':
                html = `<pre class="markup markup_codeblock">${html}</pre>`;
                break;

            case 'Spoiler':
                html = `<span class="markup markup_spoiler">${html}</span>`;
                break;

            case 'Color':
                html = `<span style="color: ${tag.color};">${html}</span>`;
                break;

            case 'RefLink':
                const targetPost = _posts[tag.id];
                if (targetPost) {
                    html +=
                        `<span class="reflink__author">` +
                        `<span class="reflink__name">${formatName(targetPost)}</span>` +
                        `<span class="reflink__tripcode">${targetPost.tripcode}</span>` +
                        `</span>`;
                }

                html = `<a class="markup markup_reflink reflink" href="#post_${tag.id}" ` +
                    `data-ref-link="${tag.id}"` +
                    `data-show-post-popup="${tag.id}">${html}</a>`;
                break;

            case 'Link':
                const className = [
                    'markup',
                    'markup_link',
                    tag.icon ? 'markup_icon_' + tag.icon : null,
                ].filter(c => c).join(' ');
                html = `<a class="${className}" href="${tag.url}" target="_blank">${html}</a>`;
                break;

            case 'Quote':
                html = `<span class="markup markup_quote">${html}</span>`;
                break;

            case 'HTML':
                html = tag.content;
                break;
        }
    });

    return html;
}
