import { Component, ChangeDetectionStrategy } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SidebarComponent } from './sidebar/sidebar.component';

@Component({
    selector: 'app-docs',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [RouterOutlet, SidebarComponent],
    template: `
        <div class="docs-layout">
            <aside class="docs-sidebar">
                <app-docs-sidebar />
            </aside>
            <main class="docs-content">
                <router-outlet />
            </main>
        </div>
    `,
    styles: [`
        .docs-layout {
            display: grid;
            grid-template-columns: 280px 1fr;
            gap: 3rem;
            max-width: 1400px;
            margin: 0 auto;
            padding: 2rem;
            min-height: calc(100vh - 80px);
            margin-top: 80px;
        }

        .docs-sidebar {
            border-right: 1px solid var(--border-subtle);
        }

        .docs-content {
            max-width: 900px;
            padding-bottom: 4rem;
        }

        @media (max-width: 1024px) {
            .docs-layout {
                grid-template-columns: 240px 1fr;
                gap: 2rem;
            }
        }

        @media (max-width: 768px) {
            .docs-layout {
                grid-template-columns: 1fr;
                gap: 1rem;
            }

            .docs-sidebar {
                border-right: none;
                border-bottom: 1px solid var(--border-subtle);
                padding-bottom: 1rem;
            }
        }
    `]
})
export class DocsComponent { }
