export enum ModType {
    Content = "Content",
    Map = "Map",
}

export class MapMod {
    is_active: boolean;
    file_name: string;
    internal_name: string;
    external_name: string;
    authors: string;
}

export class InnerContent {
    internal_name: string;
    external_name: string;
    authors: string;
    brand: string;
    name: string;
}

export class ContentMod {
    is_active: boolean;
    file_name: string;
    inner_content: InnerContent[];
}