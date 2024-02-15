import { NetworkStatus, NodeStatus } from "./structs";

export class NodeConnection {
    constructor(private mana = 0) { }

    async getPage(path: string): Promise<string> {
        console.log(`requesting to ${path}`)
        await new Promise((res) => setTimeout(res, 100));
        try {
            return (await fetch(`/assets/${path}`)).text();
        } catch (e) {
            return `<h1>404 Page not found</h1><p>Sorry, don't know page ${path}`;
        }
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