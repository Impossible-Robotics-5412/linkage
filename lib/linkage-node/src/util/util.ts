/**
 * Makes sure a value is within a certain range. If it exceeds the min or max value it limits it to min or max respectively.
 * @param value The value to clamp.
 * @param min The smallest value that is allowed.
 * @param max The largest value that is allowed.
 */
export function clamp(value: number, min: number, max: number): number {
    return Math.min(max, Math.max(min, value));
}

// FIXME: Documentation
export function clampMotorValue(value: number): number {
    return Math.min(1.0, Math.max(-1.0, value));
}

// FIXME: Documentation
export function mapRange(value: number, min1: number, max1: number, min2: number, max2: number): number {
    return (value - min1) / (max1 - min1) * (max2 - min2) + min2;
}
