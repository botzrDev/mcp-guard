import { Component, ChangeDetectionStrategy } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
    selector: 'app-docs',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [RouterOutlet],
    template: `
    <div class="docs-layout">
      <aside class="docs-sidebar">
        <!-- Placeholder for sidebar -->
        <h3>Documentation</h3>
      </aside>
      <div class="docs-content">
        <router-outlet />
      </div>
    </div>
  `,
    styles: [`
    .docs-layout {
      display: grid;
      grid-template-columns: 250px 1fr;
      gap: 2rem;
      max-width: 1280px;
      margin: 0 auto;
      padding: 2rem;
      min-height: 80vh;
      margin-top: 80px; /* Offset for fixed header */
    }
    
    .docs-sidebar {
      border-right: 1px solid var(--border-subtle);
    }
  `]
})
export class DocsComponent { }
