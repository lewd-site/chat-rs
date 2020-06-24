import { File } from '../types';

interface GalleryEntry {
    readonly top: number;
    readonly left: number;
    readonly width: number;
    readonly height: number;
    readonly file: File;
}

interface Gallery {
    readonly width: number;
    readonly height: number;
    readonly entries: GalleryEntry[];
}

const SIZE = 240;
const SIZE_4 = 240 * 1.125;
const SIZE_5 = 240 * 1.25;
const GAP = 8;

export function gallery(files: File[]): Gallery {
    switch (files.length) {
        case 0: {
            return {
                width: 0,
                height: 0,
                entries: [],
            };
        }

        case 1: {
            const file = files[0];
            const { width: w, height: h } = file;
            const s = Math.min(1, SIZE / w!, SIZE / h!);

            const sw = Math.floor(s * w!);
            const sh = Math.floor(s * h!);

            return {
                width: sw,
                height: sh,
                entries: [{
                    top: 0,
                    left: 0,
                    width: sw,
                    height: sh,
                    file,
                }]
            };
        }

        case 2: {
            const [file1, file2] = files;

            const { width: w1, height: h1 } = file1;
            const { width: w2, height: h2 } = file2;

            const s1 = Math.min(1, SIZE / w1!, SIZE / h1!);
            const s2 = Math.min(1, SIZE / w2!, SIZE / h2!);

            const sw1 = s1 * w1!;
            const sh1 = s1 * h1!;

            const sw2 = s2 * w2!;
            const sh2 = s2 * h2!;

            if (sw1! + sw2! > sh1! + sh2!) {
                const sw = Math.floor(Math.min(sw1, sw2));

                const sh1 = Math.floor(h1! * sw / w1!);
                const sh2 = Math.floor(h2! * sw / w2!);

                return {
                    width: sw,
                    height: sh1 + GAP + sh2,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw,
                            height: sh1,
                            file: file1,
                        },
                        {
                            top: sh1 + GAP,
                            left: 0,
                            width: sw,
                            height: sh2,
                            file: file2,
                        },
                    ],
                };
            } else {
                const sh = Math.floor(Math.min(sh1, sh2));

                const sw1 = Math.floor(w1! * sh / h1!);
                const sw2 = Math.floor(w2! * sh / h2!);

                return {
                    width: sw1 + GAP + sw2,
                    height: sh,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw1,
                            height: sh,
                            file: file1,
                        },
                        {
                            top: 0,
                            left: sw1 + GAP,
                            width: sw2,
                            height: sh,
                            file: file2,
                        },
                    ],
                };
            }
        }

        case 3: {
            const [file1, file2, file3] = files;

            const { width: w1, height: h1 } = file1;
            const { width: w2, height: h2 } = file2;
            const { width: w3, height: h3 } = file3;

            const s1 = Math.min(1, SIZE / w1!, SIZE / h1!);
            const sw1 = Math.floor(s1 * w1!);
            const sh1 = Math.floor(s1 * h1!);

            if (sw1 < sh1) {
                const sh = Math.floor((sh1 - GAP) / 2);
                const s2 = Math.min(1, sh / w2!, sh / h2!);
                const s3 = Math.min(1, sh / w3!, sh / h3!);
                const sw = Math.floor(Math.min(s2 * w2!, s3 * w3!));

                return {
                    width: sw1 + GAP + sw,
                    height: sh1,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw1,
                            height: sh1,
                            file: file1,
                        },
                        {
                            top: 0,
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file2,
                        },
                        {
                            top: sh + GAP,
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file3,
                        },
                    ],
                };
            } else {
                const sw = Math.floor((sw1 - GAP) / 2);
                const s2 = Math.min(1, sw / w2!, sw / h2!);
                const s3 = Math.min(1, sw / w3!, sw / h3!);
                const sh = Math.floor(Math.min(s2 * w2!, s3 * w3!));

                return {
                    width: sw1,
                    height: sh1 + GAP + sh,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw1,
                            height: sh1,
                            file: file1,
                        },
                        {
                            top: sh1 + GAP,
                            left: 0,
                            width: sw,
                            height: sh,
                            file: file2,
                        },
                        {
                            top: sh1 + GAP,
                            left: sw + GAP,
                            width: sw,
                            height: sh,
                            file: file3,
                        },
                    ],
                };
            }
        }

        case 4: {
            const [file1, file2, file3, file4] = files;

            const { width: w1, height: h1 } = file1;
            const { width: w2, height: h2 } = file2;
            const { width: w3, height: h3 } = file3;
            const { width: w4, height: h4 } = file4;

            const s1 = Math.min(1, SIZE_4 / w1!, SIZE_4 / h1!);
            const sw1 = Math.floor(s1 * w1!);
            const sh1 = Math.floor(s1 * h1!);

            if (sw1 < sh1) {
                const sh = Math.floor((sh1 - 2 * GAP) / 3);
                const s2 = Math.min(1, sh / w2!, sh / h2!);
                const s3 = Math.min(1, sh / w3!, sh / h3!);
                const s4 = Math.min(1, sh / w4!, sh / h4!);
                const sw = Math.floor(Math.min(s2 * w2!, s3 * w3!, s4 * w4!));

                return {
                    width: sw1 + GAP + sw,
                    height: sh1,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw1,
                            height: sh1,
                            file: file1,
                        },
                        {
                            top: 0,
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file2,
                        },
                        {
                            top: sh + GAP,
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file3,
                        },
                        {
                            top: 2 * (sh + GAP),
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file4,
                        },
                    ],
                };
            } else {
                const sw = Math.floor((sw1 - 2 * GAP) / 3);
                const s2 = Math.min(1, sw / w2!, sw / h2!);
                const s3 = Math.min(1, sw / w3!, sw / h3!);
                const s4 = Math.min(1, sw / w4!, sw / h4!);
                const sh = Math.floor(Math.min(s2 * w2!, s3 * w3!, s4 * w4!));

                return {
                    width: sw1,
                    height: sh1 + GAP + sh,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw1,
                            height: sh1,
                            file: file1,
                        },
                        {
                            top: sh1 + GAP,
                            left: 0,
                            width: sw,
                            height: sh,
                            file: file2,
                        },
                        {
                            top: sh1 + GAP,
                            left: sw + GAP,
                            width: sw,
                            height: sh,
                            file: file3,
                        },
                        {
                            top: sh1 + GAP,
                            left: 2 * (sw + GAP),
                            width: sw,
                            height: sh,
                            file: file4,
                        },
                    ],
                };
            }
        }

        case 5: {
            const [file1, file2, file3, file4, file5] = files;

            const { width: w1, height: h1 } = file1;
            const { width: w2, height: h2 } = file2;
            const { width: w3, height: h3 } = file3;
            const { width: w4, height: h4 } = file4;
            const { width: w5, height: h5 } = file5;

            const s1 = Math.min(1, SIZE_5 / w1!, SIZE_5 / h1!);
            const sw1 = Math.floor(s1 * w1!);
            const sh1 = Math.floor(s1 * h1!);

            if (sw1 < sh1) {
                const sh = Math.floor((sh1 - 3 * GAP) / 4);
                const s2 = Math.min(1, sh / w2!, sh / h2!);
                const s3 = Math.min(1, sh / w3!, sh / h3!);
                const s4 = Math.min(1, sh / w4!, sh / h4!);
                const s5 = Math.min(1, sh / w5!, sh / h5!);
                const sw = Math.floor(Math.min(s2 * w2!, s3 * w3!, s4 * w4!, s5 * w5!));

                return {
                    width: sw1 + GAP + sw,
                    height: sh1,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw1,
                            height: sh1,
                            file: file1,
                        },
                        {
                            top: 0,
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file2,
                        },
                        {
                            top: sh + GAP,
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file3,
                        },
                        {
                            top: 2 * (sh + GAP),
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file4,
                        },
                        {
                            top: 3 * (sh + GAP),
                            left: sw1 + GAP,
                            width: sw,
                            height: sh,
                            file: file5,
                        },
                    ],
                };
            } else {
                const sw = Math.floor((sw1 - 3 * GAP) / 4);
                const s2 = Math.min(1, sw / w2!, sw / h2!);
                const s3 = Math.min(1, sw / w3!, sw / h3!);
                const s4 = Math.min(1, sw / w4!, sw / h4!);
                const s5 = Math.min(1, sw / w5!, sw / h5!);
                const sh = Math.floor(Math.min(s2 * w2!, s3 * w3!, s4 * w4!, s5 * w5!));

                return {
                    width: sw1,
                    height: sh1 + GAP + sh,
                    entries: [
                        {
                            top: 0,
                            left: 0,
                            width: sw1,
                            height: sh1,
                            file: file1,
                        },
                        {
                            top: sh1 + GAP,
                            left: 0,
                            width: sw,
                            height: sh,
                            file: file2,
                        },
                        {
                            top: sh1 + GAP,
                            left: sw + GAP,
                            width: sw,
                            height: sh,
                            file: file3,
                        },
                        {
                            top: sh1 + GAP,
                            left: 2 * (sw + GAP),
                            width: sw,
                            height: sh,
                            file: file4,
                        },
                        {
                            top: sh1 + GAP,
                            left: 3 * (sw + GAP),
                            width: sw,
                            height: sh,
                            file: file5,
                        },
                    ],
                };
            }
        }
    }

    return { width: 0, height: 0, entries: [] };
}
