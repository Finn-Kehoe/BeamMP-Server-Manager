export enum ModType {
    Vehicle,
    Map,
    None
}

export class Mod {
    mod_type: ModType;
    is_active: boolean;
    internal_name: string;
    file_name: string;
    details: Map<string, string>;
}