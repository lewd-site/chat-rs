import jwtDecode from 'jwt-decode';

import config from '../config';

export interface TokenData {
  readonly user_uuid: string;
  readonly user_name: string;
  readonly user_email: string;
  readonly iat: number;
  readonly nbf: number;
  readonly exp: number;
}

class Deferred<T> {
  public readonly promise: Promise<T>;
  private _resolve?: (value?: T | PromiseLike<T>) => void;
  private _reject?: (reason?: unknown) => void;

  public constructor() {
    this.promise = new Promise<T>((resolve, reject) => {
      this._resolve = resolve;
      this._reject = reject;
    });
  }

  public resolve = (value: T) => {
    if (typeof this._resolve !== 'undefined') {
      this._resolve(value);
    }
  };

  public reject = (reason: unknown) => {
    if (typeof this._reject !== 'undefined') {
      this._reject(reason);
    }
  };
}

export class Sso {
  private readonly element: HTMLIFrameElement;
  private readonly requests: { [id: number]: Deferred<TokenData | null> } = {};

  private id = 0;
  private _accessToken: string | null = null;
  private _refreshToken: string | null = null;
  private _accessTokenData: TokenData | null = null;
  private _loaded = false;

  public get hasAccessToken(): boolean {
    return this._accessToken !== null;
  }

  public get hasRefreshToken(): boolean {
    return this._refreshToken !== null;
  }

  public get accessToken(): string | null {
    return this._accessToken;
  }

  public get accessTokenData(): TokenData | null {
    return this._accessTokenData;
  }

  public get hasExpired(): boolean {
    if (!this.hasAccessToken || !this.accessTokenData) {
      return true;
    }

    const now = Date.now();
    const exp = this.accessTokenData.exp * 1000;

    return exp <= now;
  }

  public get loaded(): boolean {
    return this._loaded;
  }

  public constructor(private readonly loadCallback?: () => void) {
    this.bind();

    this.element = document.createElement('iframe');
    this.element.setAttribute('hidden', '');
    this.element.src = config.ssoOrigin;
    document.body.appendChild(this.element);
  }

  public bind: () => void = () => {
    window.addEventListener('message', this.handleMessage);
  };

  public unbind: () => void = () => {
    window.removeEventListener('message', this.handleMessage);
  };

  private handleMessage = (e: MessageEvent) => {
    if (e.data === 'sso_loaded') {
      this._loaded = true;

      if (this.loadCallback) {
        this.loadCallback();
      }
    } else if (e.data.command === 'set_token') {
      this._accessToken = e.data.access_token || null;
      this._refreshToken = e.data.refresh_token || null;

      if (this.accessToken) {
        this._accessTokenData = jwtDecode(this.accessToken);
      } else {
        this._accessTokenData = null;
      }

      if (e.data.id) {
        const request = this.requests[e.data.id];
        if (request) {
          request.resolve(this.accessTokenData);
        }

        delete this.requests[e.data.id];
      }
    } else if (e.data.command === 'error') {
      if (e.data.id) {
        const request = this.requests[e.data.id];
        if (request) {
          request.reject(e.data.error);
        }

        delete this.requests[e.data.id];
      }
    }
  };

  public get = (): Promise<TokenData | null> => {
    const id = ++this.id;

    this.element.contentWindow?.postMessage(
      {
        id,
        command: 'get_token',
      },
      config.ssoOrigin,
    );

    const request = new Deferred<TokenData | null>();
    this.requests[id] = request;
    return request.promise;
  };

  public loginByEmail = (email: string, password: string): Promise<TokenData | null> => {
    const id = ++this.id;

    this.element.contentWindow?.postMessage(
      {
        id,
        command: 'refresh_token',
        email,
        password,
      },
      config.ssoOrigin,
    );

    const request = new Deferred<TokenData | null>();
    this.requests[id] = request;
    return request.promise;
  };

  public loginByName = (name: string, password: string): Promise<TokenData | null> => {
    const id = ++this.id;

    this.element.contentWindow?.postMessage(
      {
        id,
        command: 'refresh_token',
        name,
        password,
      },
      config.ssoOrigin,
    );

    const request = new Deferred<TokenData | null>();
    this.requests[id] = request;
    return request.promise;
  };

  public refreshByEmail = (email: string): Promise<TokenData | null> => {
    const id = ++this.id;

    this.element.contentWindow?.postMessage(
      {
        id,
        command: 'refresh_token',
        email,
        refresh_token: this._refreshToken,
      },
      config.ssoOrigin,
    );

    const request = new Deferred<TokenData | null>();
    this.requests[id] = request;
    return request.promise;
  };

  public refreshByName = (name: string): Promise<TokenData | null> => {
    const id = ++this.id;

    this.element.contentWindow?.postMessage(
      {
        id,
        command: 'refresh_token',
        name,
        refresh_token: this._refreshToken,
      },
      config.ssoOrigin,
    );

    const request = new Deferred<TokenData | null>();
    this.requests[id] = request;
    return request.promise;
  };
}

export default Sso;
