import { addPost, setPosts } from './stores/posts';
import { Post } from './types';
import utils from './utils';

interface WsPostCreated {
    readonly event: 'post_created',
    readonly data: {
        readonly item: Post,
    },
}

type WsEvent = WsPostCreated;

function isWsEvent(data: any): data is WsEvent {
    return (data as WsEvent).event !== undefined;
}

export class Ws {
    private ws: null | WebSocket = null;

    public constructor(private readonly url: string) {
        this.open();
    }

    private open = () => {
        this.ws = new WebSocket(this.url);
        this.ws.addEventListener('open', this.onOpen);
        this.ws.addEventListener('close', this.onClose);
        this.ws.addEventListener('message', this.onMessage);
        this.ws.addEventListener('error', this.onError);
    };

    private close = () => {
        this.ws?.close();
        this.ws = null;
    };

    private onOpen = async (e: Event) => {
        // Reload latest posts after websocket connected.
        const posts = await window.api!.getLatestPosts();

        const scroll = utils.isAtBottom();
        if (scroll) {
            setTimeout(utils.scrollToBottom);
        }

        setPosts(posts);
    };

    private onClose = (e: CloseEvent) => {
        if (e.code === 1000) {
            return;
        }

        setTimeout(this.open, 10000);
    };

    private onMessage = (e: MessageEvent) => {
        const message = JSON.parse(e.data);
        if (!isWsEvent(message)) {
            return;
        }

        switch (message.event) {
            case 'post_created': {
                const scroll = utils.isAtBottom();
                if (scroll) {
                    setTimeout(utils.scrollToBottom);
                }

                addPost(message.data.item);
                break;
            }
        }
    };

    private onError = (e: Event) => {
        this.close();
    };
}

export default Ws;
