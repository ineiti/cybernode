import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable, interval } from 'rxjs';
import { NetworkStatus, NodeStatus } from '../lib/structs';
import { NodeConnection } from '../lib/connect';

@Injectable({
  providedIn: 'root'
})
export class ConnectorService {
  private _networkStatus: BehaviorSubject<NetworkStatus> = new BehaviorSubject({
    users_total: 0,
    users_active: 0,
  });
  public readonly networkStatus: Observable<NetworkStatus> = this._networkStatus.asObservable();

  private _nodeStatus: BehaviorSubject<NodeStatus> = new BehaviorSubject({
    name: "undefined",
    mana: 0,
  });
  public readonly nodeStatus: Observable<NodeStatus> = this._nodeStatus.asObservable();

  connection = new NodeConnection();

  constructor() {
    interval(1000).subscribe(async () => {
      this._nodeStatus.next(await this.connection.getNodeStatus());
      this._networkStatus.next(await this.connection.getNetworkStatus());
    });
  }

  async getPage(url: string): Promise<string> {
    return this.connection.getPage(url);
  }
}
