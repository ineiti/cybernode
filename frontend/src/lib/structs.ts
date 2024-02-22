export interface NetworkStatus {
    users_total: number,
    users_active: number,
}

export interface NodeStatus {
    mana: number,
    name: string,
}

export interface INodeConnection {
    getText(path: string): Promise<string>;
    getBlob(path: string): Promise<Blob>;
    getNetworkStatus(): Promise<NetworkStatus>;
    getNodeStatus(): Promise<NodeStatus>;
}