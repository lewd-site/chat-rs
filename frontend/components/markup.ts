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

    if (m.type === 'Text') {
        return escapeHtml(m.text);
    } else if (m.type === 'Tag') {
        let html = markup(m.children);
        switch (m.tag.type) {
            case 'Bold':
                return `<strong class="markup markup_bold">${html}</strong>`;

            case 'Italic':
                return `<em class="markup markup_italic">${html}</em>`;

            case 'Underline':
                return `<span class="markup markup_underline">${html}</span>`;

            case 'Strike':
                return `<del class="markup markup_strike">${html}</del>`;

            case 'Superscript':
                return `<sup class="markup markup_superscript">${html}</sup>`;

            case 'Subscript':
                return `<sub class="markup markup_subscript">${html}</sub>`;

            case 'Code':
                return `<pre class="markup markup_code">${html}</pre>`;

            case 'CodeBlock':
                return `<pre class="markup markup_codeblock">${html}</pre>`;

            case 'Spoiler':
                return `<span class="markup markup_spoiler">${html}</span>`;

            case 'Color':
                return `<span style="color: ${m.tag.color};">${html}</span>`;

            case 'RefLink':
                const targetPost = _posts[m.tag.id];
                if (targetPost) {
                    html +=
                        `<span class="reflink__author">` +
                        `<span class="reflink__name">${formatName(targetPost)}</span>` +
                        `<span class="reflink__tripcode">${targetPost.tripcode}</span>` +
                        `</span>`;
                }

                return `<a class="markup markup_reflink reflink" href="#post_${m.tag.id}" ` +
                    `data-ref-link="${m.tag.id}"` +
                    `data-show-post-popup="${m.tag.id}">${html}</a>`;

            case 'Link':
                const className = [
                    'markup',
                    'markup_link',
                    m.tag.icon ? 'markup_icon_' + m.tag.icon : null,
                ].filter(c => c).join(' ');
                return `<a class="${className}" href="${m.tag.url}" target="_blank">${html}</a>`;

            case 'Quote':
                return `<span class="markup markup_quote">${html}</span>`;

            case 'HTML':
                return m.tag.content;

            default:
                return '';
        }
    } else {
        return '';
    }
}
