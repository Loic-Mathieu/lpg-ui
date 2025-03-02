import {path} from "@tauri-apps/api";
import {DialogFilter} from "@tauri-apps/plugin-dialog";

/**
 * Interface that holds file info
 */
export interface PathData {
    path: string;
    file: string | undefined;
    uri: string;
    name: string | undefined;
    extension: string | undefined;
}

const SEP = path.sep();
const EXT = '.';
const PKG_EXTENSION = 'zip';

/**
 * Generate a {@link PathData} from a raw path
 * @param rawPath the file path
 */
export function getPathData(rawPath: string): PathData {
    const splitPath = rawPath.split(SEP);
    const file = splitPath.pop();

    if (!file || !file.includes(EXT)) {
        return {
            path: rawPath,
            uri: rawPath,
            file: undefined,
            name: undefined,
            extension: undefined
        }
    }

    const uri = splitPath.join(SEP);

    const splitFile = file.split(EXT);
    const extension = splitFile.pop();
    const name = splitFile.join(EXT);

    return {
        path: rawPath,
        file,
        uri,
        name,
        extension
    }
}

/**
 * Create a file path from an array of strings
 * @param pathBits
 */
export function joinPath(...pathBits: (string | undefined)[]): string {
    return pathBits.filter(value => !!value).join(SEP);
}

/**
 * Types of file system filters
 */
export enum FilterType {
    PACKAGE = 'Package',
    PICTURES = 'Pictures',
}

/**
 * File system filters
 */
export const filters: Record<FilterType, DialogFilter[]> = {
    [FilterType.PACKAGE]: [
        {name: FilterType.PACKAGE, extensions: [PKG_EXTENSION]}
    ],
    [FilterType.PICTURES]: [
        {name: FilterType.PICTURES, extensions: ['png', 'jpg', 'jpeg']}
    ],
};

/**
 * Return true if package has the correct type
 * @param pkg the package
 */
export function isPackage(pkg: PathData) {
    return pkg.extension && pkg.extension === PKG_EXTENSION
}