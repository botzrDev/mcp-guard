import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-about',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <div class="page-container">
      <h1>About Us</h1>
      <p>Learn more about the team behind MCP Guard.</p>
    </div>
  `,
    styles: [`
    .page-container {
      max-width: 800px;
      margin: 0 auto;
      padding: 2rem;
      margin-top: 80px;
    }
  `]
})
export class AboutComponent { }
