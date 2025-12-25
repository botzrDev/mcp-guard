import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-blog',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <div class="page-container">
      <h1>Blog</h1>
      <p>Latest news and articles.</p>
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
export class BlogComponent { }
