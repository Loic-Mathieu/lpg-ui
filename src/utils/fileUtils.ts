import {path} from "@tauri-apps/api";
import {DialogFilter} from "@tauri-apps/plugin-dialog";

export interface PathData {
    path: string;
    file: string | undefined;
    uri: string;
    name: string | undefined;
    extension: string | undefined;
}

const SEP = path.sep();
const EXT = '.';

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
    const name = splitFile.pop();

    return {
        path: rawPath,
        file,
        uri,
        name,
        extension
    }
}

export function joinPath(...pathBits: (string | undefined)[]): string {
    return pathBits.filter(value => !!value).join(SEP);
}

export enum FilterType {
    PACKAGE = 'Package',
    PICTURES = 'Pictures',
}

export const filters: Record<FilterType, DialogFilter[]> = {
    [FilterType.PACKAGE]: [
        {name: FilterType.PACKAGE, extensions: ['zip']}
    ],
    [FilterType.PICTURES]: [
        {name: FilterType.PICTURES, extensions: ['png', 'jpg', 'jpeg']}
    ],
};