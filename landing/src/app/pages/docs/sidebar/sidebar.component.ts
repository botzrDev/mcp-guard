import { Component, inject, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink, RouterLinkActive } from '@angular/router';
import { DocsService, DocCategory } from '../docs.service';

@Component({
    selector: 'app-docs-sidebar',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [CommonModule, RouterLink, RouterLinkActive],
    template: `
        <nav class="sidebar">
            <div class="sidebar-header">
                <h2>Documentation</h2>
                <span class="version">v1.0</span>
            </div>

            @for (category of categories; track category.slug) {
                <div class="category">
                    <h3 class="category-title">{{ category.name }}</h3>
                    <ul class="nav-list">
                        @for (page of category.pages; track page.slug) {
                            <li>
                                <a
                                    [routerLink]="['/docs', page.slug]"
                                    routerLinkActive="active"
                                    class="nav-link"
                                >
                                    {{ page.title }}
                                </a>
                            </li>
                        }
                    </ul>
                </div>
            }
        </nav>
    `,
    styles: [`
        .sidebar {
            position: sticky;
            top: 100px;
            padding-right: 1.5rem;
            max-height: calc(100vh - 120px);
            overflow-y: auto;
        }

        .sidebar-header {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            margin-bottom: 1.5rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border-subtle);

            h2 {
                font-size: 1.25rem;
                font-weight: 600;
                color: var(--text-primary);
                margin: 0;
            }

            .version {
                font-size: 0.75rem;
                padding: 0.25rem 0.5rem;
                background: var(--accent);
                color: var(--bg-primary);
                border-radius: 9999px;
                font-weight: 500;
            }
        }

        .category {
            margin-bottom: 1.5rem;
        }

        .category-title {
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: var(--text-muted);
            margin-bottom: 0.75rem;
        }

        .nav-list {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .nav-link {
            display: block;
            padding: 0.5rem 0.75rem;
            margin: 0.125rem 0;
            color: var(--text-secondary);
            text-decoration: none;
            border-radius: 6px;
            font-size: 0.9rem;
            transition: all 0.2s ease;

            &:hover {
                color: var(--text-primary);
                background: var(--surface-elevated);
            }

            &.active {
                color: var(--accent);
                background: rgba(var(--accent-rgb), 0.1);
                font-weight: 500;
            }
        }

        /* Custom scrollbar */
        .sidebar::-webkit-scrollbar {
            width: 4px;
        }

        .sidebar::-webkit-scrollbar-track {
            background: transparent;
        }

        .sidebar::-webkit-scrollbar-thumb {
            background: var(--border-subtle);
            border-radius: 2px;
        }
    `]
})
export class SidebarComponent {
    private docsService = inject(DocsService);
    categories: DocCategory[] = this.docsService.getCategories();
}
