import { NetworkStatus, NodeStatus } from "./structs";

export class NodeConnection {
    constructor(private mana = 0) {
    }

    async getNetworkStatus(): Promise<NetworkStatus> {
        return new Promise((res) => {
            let total = Math.floor(Math.random() * 20);
            setTimeout(() => res({
                users_total: total,
                users_active: Math.min(Math.floor(Math.random() * 5), total)
            }), 500 + Math.random() * 500);
        })
    }

    async getNodeStatus(): Promise<NodeStatus> {
        return new Promise((res) => setTimeout(() => res({
            name: "personal",
            mana: this.mana++,
        }), 500 + Math.random() * 500));
    }
}