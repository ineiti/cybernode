import { Component } from '@angular/core';
import { ConnectorService } from '../../connector.service';
import { NetworkStatus, NodeStatus } from '../../../lib/structs';
import { Observable } from 'rxjs';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-status',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './status.component.html',
  styleUrl: './status.component.scss'
})
export class StatusComponent {
  networkStatus: Observable<NetworkStatus>;
  nodeStatus: Observable<NodeStatus>;

  constructor(private connector: ConnectorService){
    this.networkStatus = connector.networkStatus;
    this.nodeStatus = connector.nodeStatus;
  }
}
