const BOTTOM_SCROLL_MARGIN = 20;

export class Utils {
    public isAtTop() {
        const scrollingElement = (document.scrollingElement || document.body);
        return scrollingElement.scrollTop === 0;
    }

    public isAtBottom() {
        const scrollingElement = (document.scrollingElement || document.body);
        const offsetHeight = (scrollingElement as any).offsetHeight;
        const { scrollTop, scrollHeight } = scrollingElement;
        return offsetHeight + scrollTop > scrollHeight - BOTTOM_SCROLL_MARGIN;
    }

    public scrollToTop() {
        const scrollingElement = (document.scrollingElement || document.body);
        scrollingElement.scrollTop = 1;
    }

    public scrollToBottom() {
        const scrollingElement = (document.scrollingElement || document.body);
        scrollingElement.scrollTop = scrollingElement.scrollHeight;
    }

    public maintainScrollBottom() {
        const scrollingElement = (document.scrollingElement || document.body);
        const scrollHeight = scrollingElement.scrollHeight;
        setTimeout(() => {
            const newScrollHeight = scrollingElement.scrollHeight;
            scrollingElement.scrollTop = newScrollHeight - scrollHeight;
        });
    }
}

export const utils = new Utils();
export default utils;

