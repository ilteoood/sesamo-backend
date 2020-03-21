export interface ActionBase {
    open(configuration: Map<string, string>): Promise<boolean>;
}
