export enum ModType {
    Vehicle = "Vehicle",
    Map = "Map",
}

export class Mod {
    mod_type: ModType;
    is_active: boolean;
    internal_name: string;
    file_name: string;
    details: Map<string, string>;
}