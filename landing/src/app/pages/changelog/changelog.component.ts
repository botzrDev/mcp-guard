import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-changelog',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <div class="page-container">
      <h1>Changelog</h1>
      <p>Latest updates and changes.</p>
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
export class ChangelogComponent { }
