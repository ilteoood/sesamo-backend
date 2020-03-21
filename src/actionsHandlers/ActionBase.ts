export interface ActionBase {
    open(configuration: any): Promise<boolean>;
}
