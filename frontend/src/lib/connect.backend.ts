import { NetworkStatus, NodeStatus } from "./structs";

export class NodeConnectionBackend {
    constructor(private mana = 0) { }

    async getText(path: string): Promise<string> {
        return "Not implemented";
    }

    async getBlob(path: string): Promise<Blob> {
        return new Blob();
    }

    async getNetworkStatus(): Promise<NetworkStatus> {
        return {
            users_total: 0,
            users_active: 0,
        };
    }

    async getNodeStatus(): Promise<NodeStatus> {
        return {
            mana: 0,
            name: "test",
        };
    }
}