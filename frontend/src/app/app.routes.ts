import { Routes, UrlSegment } from '@angular/router';
import { PageComponent } from './blob/page/page.component';
import { NodeComponent } from './blob/node/node.component';
import { WalletComponent } from './blob/wallet/wallet.component';

export const routes: Routes = [
    { path: '', redirectTo: '/page/cybernode', pathMatch: 'full' },
    { path: 'node/:id', component: NodeComponent },
    { path: 'wallet/:id', component: WalletComponent },
    {
        // Create a matcher for "/page" which joins all the sub urls, because the 'url' parameter
        // of the PageComponent can have many urls.
        matcher: (url) => {
            if (url.length >= 1 && url[0].path === "page") {
                return { consumed: url, posParams: { url: new UrlSegment(url.slice(1).join("/"), {}) } };
            }

            return null;
        },
        component: PageComponent
    },
];
