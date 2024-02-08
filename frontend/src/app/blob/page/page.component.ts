import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-page',
  standalone: true,
  imports: [],
  templateUrl: './page.component.html',
  styleUrl: './page.component.scss'
})
export class PageComponent {
  @Input() url = "";
  html = "<h1>Setting up</h1>";

  async ngOnInit() {
    console.log(`URL is: ${this.url}`);
    switch (this.url) {
      case "cybernode":
        this.html = `
<h1>Welcome to My Technical Website</h1>
  <p>This is a technical website showcasing innovative ideas and solutions in the field of technology.</p>
  <p>We are passionate about empowering businesses and individuals to leverage the power of technology.</p>
  <p>Our team of experienced experts is committed to providing you with the best possible solutions to your technical
    challenges.</p>
        `;
        break;
      default:
        this.html = `<h1>404 Page not found</h1><p>Sorry, don't know page ${this.url}`;
    }
  }
}
