export interface ActionBase {
    open(configuration: Record<string, string>): Promise<boolean>;
}
