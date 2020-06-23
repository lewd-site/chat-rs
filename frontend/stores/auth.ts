import { writable, derived } from 'svelte/store';
import { TokenData } from '../services/sso';

export const showAuthModal = writable<boolean>(false);

export const token = writable<TokenData | null>(null);

export const userUuid = derived(token, token => token?.user_uuid);
