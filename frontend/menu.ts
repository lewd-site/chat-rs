import { writable } from 'svelte/store';

export class Menu {
    private readonly menuBar: HTMLElement;
    private readonly menuInner: HTMLElement;
    private readonly showButton: HTMLElement;
    private readonly hideButton: HTMLElement;
    private readonly nameInput: HTMLInputElement;
    private readonly tripcodeInput: HTMLInputElement;

    private readonly isVisible = writable<boolean>(true);
    private readonly name = writable<string>("");
    private readonly tripcode = writable<string>("");

    private isVisibleUnsubscriber?: () => void;
    private nameUnsubscriber?: () => void;
    private tripcodeUnsubscriber?: () => void;

    public constructor(private readonly menuElement: HTMLElement) {
        const menuBar = menuElement.querySelector('.menu__bar');
        const menuInner = menuElement.querySelector('.menu__inner');
        const showButton = menuElement.querySelector('.menu__show');
        const hideButton = menuElement.querySelector('.menu__hide');
        const nameInput = menuElement.querySelector('input[name="name"]');
        const tripcodeInput = menuElement.querySelector('input[name="tripcode"]');

        if (!menuBar || !(menuBar instanceof HTMLElement)) {
            throw new Error('.menu__bar not found');
        }

        if (!menuInner || !(menuInner instanceof HTMLElement)) {
            throw new Error('.menu__inner not found');
        }

        if (!showButton || !(showButton instanceof HTMLElement)) {
            throw new Error('.menu__show not found');
        }

        if (!hideButton || !(hideButton instanceof HTMLElement)) {
            throw new Error('.menu__hide not found');
        }

        if (!nameInput || !(nameInput instanceof HTMLInputElement)) {
            throw new Error('input[name="name"]');
        }

        if (!tripcodeInput || !(tripcodeInput instanceof HTMLInputElement)) {
            throw new Error('input[name="tripcode"]');
        }

        this.menuBar = menuBar;
        this.menuInner = menuInner;
        this.showButton = showButton;
        this.hideButton = hideButton;
        this.nameInput = nameInput;
        this.tripcodeInput = tripcodeInput;

        const name = localStorage.getItem('settings.name');
        const tripcode = localStorage.getItem('settings.tripcode');

        if (name !== null) {
            this.name.set(name);
        }

        if (tripcode !== null) {
            this.tripcode.set(tripcode);
        }

        this.bind();
        this.hide();
    }

    public bind = () => {
        this.isVisibleUnsubscriber = this.isVisible.subscribe(isVisible => {
            if (isVisible) {
                this.menuBar.setAttribute('hidden', '');
                this.menuInner.removeAttribute('hidden');
            } else {
                this.menuBar.removeAttribute('hidden');
                this.menuInner.setAttribute('hidden', '');
            }
        });

        this.nameUnsubscriber = this.name.subscribe(name => {
            this.nameInput.value = name;
            localStorage['settings.name'] = name;
        });

        this.tripcodeUnsubscriber = this.tripcode.subscribe(tripcode => {
            this.tripcodeInput.value = tripcode;
            localStorage['settings.tripcode'] = tripcode;
        });

        this.showButton.addEventListener('click', this.onShowClick);
        this.hideButton.addEventListener('click', this.onHideClick);
        this.nameInput.addEventListener('change', this.onNameChange);
        this.tripcodeInput.addEventListener('change', this.onTripcodeChange);
    }

    public unbind = () => {
        if (this.isVisibleUnsubscriber) {
            this.isVisibleUnsubscriber();
        }

        if (this.nameUnsubscriber) {
            this.nameUnsubscriber();
        }

        if (this.tripcodeUnsubscriber) {
            this.tripcodeUnsubscriber();
        }

        this.showButton.removeEventListener('click', this.onShowClick);
        this.hideButton.removeEventListener('click', this.onHideClick);
        this.nameInput.removeEventListener('change', this.onNameChange);
        this.tripcodeInput.removeEventListener('change', this.onTripcodeChange);
    }

    private onShowClick = (e: Event) => {
        e.preventDefault();
        this.show();
    }

    private onHideClick = (e: Event) => {
        e.preventDefault();
        this.hide();
    }

    private onNameChange = (e: Event) => {
        if (e.target instanceof HTMLInputElement) {
            this.name.set(e.target.value);
        }
    }

    private onTripcodeChange = (e: Event) => {
        if (e.target instanceof HTMLInputElement) {
            this.tripcode.set(e.target.value);
        }
    }

    public show = () => {
        this.isVisible.set(true);
    }

    public hide = () => {
        this.isVisible.set(false);
    }
}

export default Menu;
