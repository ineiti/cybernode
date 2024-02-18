import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Component, Input, NgZone } from '@angular/core';
import { ConnectorService } from '../../connector.service';
import { PreparePage } from '../../../lib/convert';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-page',
  standalone: true,
  imports: [],
  templateUrl: './page.component.html',
  styleUrl: './page.component.scss'
})
export class PageComponent {
  @Input() url = "";
  converter?: PreparePage;

  constructor(private connection: ConnectorService, private router: Router) {
    (window as any)['nextPage'] = (p: string) => this.nextPage(p);
    this.converter = new PreparePage((url: string) => {return this.connection.getPage(url)},
    (url: string) => {return this.connection.getBlob(url)});
  }

  async ngOnChanges() {
    console.log(`Loading page ${this.url}`);
    document.getElementById('cnpage')?.replaceChildren(await this.converter!.convert(this.url));
  }

  nextPage(page: string) {
    console.log(`Navigating to ${page}`)
    this.router.navigate([`/page/${page}`]);
  }
}
