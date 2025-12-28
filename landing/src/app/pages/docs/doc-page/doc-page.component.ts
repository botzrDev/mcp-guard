import { Component, inject, ChangeDetectionStrategy, signal, OnInit, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, Router } from '@angular/router';
import { Subject, takeUntil } from 'rxjs';
import { DocsService, DocPage } from '../docs.service';
import { MarkdownComponent } from '../../../shared/markdown/markdown.component';

@Component({
    selector: 'app-doc-page',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [CommonModule, MarkdownComponent],
    template: `
        @if (doc()) {
            <article class="doc-page">
                <div class="doc-meta">
                    <span class="category">{{ doc()!.category }}</span>
                </div>
                <app-markdown [content]="doc()!.content" />
            </article>
        } @else {
            <div class="not-found">
                <h1>Page Not Found</h1>
                <p>The documentation page you're looking for doesn't exist.</p>
                <a href="/docs/quickstart" class="back-link">Go to Quick Start</a>
            </div>
        }
    `,
    styles: [`
        .doc-page {
            animation: fadeIn 0.3s ease;
        }

        @keyframes fadeIn {
            from {
                opacity: 0;
                transform: translateY(10px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .doc-meta {
            margin-bottom: 1rem;
        }

        .category {
            display: inline-block;
            font-size: 0.75rem;
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: var(--accent);
            background: rgba(var(--accent-rgb), 0.1);
            padding: 0.25rem 0.75rem;
            border-radius: 9999px;
        }

        .not-found {
            text-align: center;
            padding: 4rem 2rem;

            h1 {
                font-size: 2rem;
                color: var(--text-primary);
                margin-bottom: 1rem;
            }

            p {
                color: var(--text-secondary);
                margin-bottom: 2rem;
            }

            .back-link {
                display: inline-block;
                padding: 0.75rem 1.5rem;
                background: var(--accent);
                color: var(--bg-primary);
                text-decoration: none;
                border-radius: 8px;
                font-weight: 500;
                transition: transform 0.2s ease;

                &:hover {
                    transform: translateY(-2px);
                }
            }
        }
    `]
})
export class DocPageComponent implements OnInit, OnDestroy {
    private route = inject(ActivatedRoute);
    private router = inject(Router);
    private docsService = inject(DocsService);
    private destroy$ = new Subject<void>();

    doc = signal<DocPage | undefined>(undefined);

    ngOnInit(): void {
        this.route.params
            .pipe(takeUntil(this.destroy$))
            .subscribe(params => {
                const slug = params['slug'];
                if (slug) {
                    this.doc.set(this.docsService.getDoc(slug));
                } else {
                    // Redirect to quickstart if no slug
                    this.router.navigate(['/docs/quickstart']);
                }
            });
    }

    ngOnDestroy(): void {
        this.destroy$.next();
        this.destroy$.complete();
    }
}
