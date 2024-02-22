import { environment } from "../environments/environment";
import { NodeConnectionBackend } from "./connect.backend";
import { NodeConnectionMock } from "./connect.mock";
import { INodeConnection, NetworkStatus, NodeStatus } from "./structs";

export class NodeConnection {
    impl?: INodeConnection;

    constructor() {
        if (environment.useBackend) {
            this.impl = new NodeConnectionBackend();
        } else {
            this.impl = new NodeConnectionMock();
        }
    }

    async getText(path: string): Promise<string> {
        return this.impl!.getText(path);
    }

    async getBlob(path: string): Promise<Blob> {
        return this.impl!.getBlob(path);
    }

    async getNetworkStatus(): Promise<NetworkStatus> {
        return this.impl!.getNetworkStatus();
    }

    async getNodeStatus(): Promise<NodeStatus> {
        return this.impl!.getNodeStatus();
    }
}