const BOTTOM_SCROLL_MARGIN = 20;

export class Utils {
  public isAtTop(): boolean {
    const scrollingElement = document.scrollingElement || document.body;
    return scrollingElement.scrollTop === 0;
  }

  public isAtBottom(): boolean {
    const scrollingElement = document.scrollingElement || document.body;
    if (!(scrollingElement instanceof HTMLElement)) {
      return false;
    }

    const offsetHeight = scrollingElement.offsetHeight;
    const { scrollTop, scrollHeight } = scrollingElement;
    return offsetHeight + scrollTop > scrollHeight - BOTTOM_SCROLL_MARGIN;
  }

  public scrollToTop(): void {
    const scrollingElement = document.scrollingElement || document.body;
    scrollingElement.scrollTop = 1;
  }

  public scrollToBottom(): void {
    const scrollingElement = document.scrollingElement || document.body;
    scrollingElement.scrollTop = scrollingElement.scrollHeight;
  }

  public maintainScrollBottom(): void {
    const scrollingElement = document.scrollingElement || document.body;
    const scrollHeight = scrollingElement.scrollHeight;
    setTimeout(() => {
      const newScrollHeight = scrollingElement.scrollHeight;
      scrollingElement.scrollTop = newScrollHeight - scrollHeight;
    });
  }

  public scrollToElement(element: Element): void {
    const rect = element.getBoundingClientRect();
    const top = rect.top + window.pageYOffset;
    const middle = top - window.innerHeight / 2 + rect.height / 2;
    window.scrollTo(0, middle);
  }
}

export const utils = new Utils();
export default utils;
