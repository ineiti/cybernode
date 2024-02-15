import { Routes, UrlSegment } from '@angular/router';
import { PageComponent } from './blob/page/page.component';
import { NodeComponent } from './blob/node/node.component';
import { WalletComponent } from './blob/wallet/wallet.component';

export const routes: Routes = [
    { path: '', redirectTo: '/page/cybernode.html', pathMatch: 'full' },
    { path: 'page/:url', component: PageComponent },
    { path: 'node/:id', component: NodeComponent },
    { path: 'wallet/:id', component: WalletComponent },
];
