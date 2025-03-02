/**
 * Check if string or array is empty
 * @param value
 */
export function isEmpty(value: string | any[]): boolean {
    return !value || value.length === 0;
}

/**
 * Check if string or array is not empty
 * @param value
 */
export function isNotEmpty(value: string | any[]): boolean {
    return !isEmpty(value);
}