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

  constructor() {
    let connection = new NodeConnection();
    interval(1000).subscribe(async () => {
      this._nodeStatus.next(await connection.getNodeStatus());
      this._networkStatus.next(await connection.getNetworkStatus());
    });
  }
}
