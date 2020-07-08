export function formatFileSize(size: number) {
    if (size === 0) {
        return '0 B';
    }

    const units = ['', 'K', 'M', 'G'];
    const unitIndex = Math.floor((31 - Math.clz32(size)) / 10);
    const unit = units[unitIndex];
    const unitSize = unitIndex > 0 ? (size / Math.pow(1024, unitIndex)).toFixed(2) : size;

    return `${unitSize} ${unit}B`;
}
