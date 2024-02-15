import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Component, Input, NgZone } from '@angular/core';
import { ConnectorService } from '../../connector.service';
import { linkToScript } from '../../../lib/convert';
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
  html: SafeHtml = "<h1>Setting up</h1>";

  constructor(private connection: ConnectorService, private sanitizer: DomSanitizer,
    private router: Router, private route: ActivatedRoute, private ngZone: NgZone) {
    (window as any)['nextPage'] = (p: string) => this.nextPage(p);
  }

  async ngOnChanges() {
    const raw = linkToScript(await this.connection.getPage(this.url), window.location.origin);
    this.html = this.sanitizer.bypassSecurityTrustHtml(raw);
  }

  nextPage(page: string) {
    this.router.navigate([`${page}`]);
  }
}
