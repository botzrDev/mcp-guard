import { Component, Input, ChangeDetectionStrategy, OnChanges, SimpleChanges, inject, PLATFORM_ID } from '@angular/core';
import { CommonModule, isPlatformBrowser } from '@angular/common';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';

@Component({
    selector: 'app-markdown',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [CommonModule],
    template: `
        <div class="markdown-content prose" [innerHTML]="renderedContent"></div>
    `,
    styles: [`
        :host {
            display: block;
        }

        .markdown-content {
            font-size: 1rem;
            line-height: 1.75;
            color: var(--text-primary);
        }

        :host ::ng-deep {
            h1 {
                font-size: 2.25rem;
                font-weight: 700;
                margin-bottom: 1.5rem;
                margin-top: 0;
                color: var(--text-primary);
                border-bottom: 1px solid var(--border-subtle);
                padding-bottom: 0.75rem;
            }

            h2 {
                font-size: 1.75rem;
                font-weight: 600;
                margin-top: 2.5rem;
                margin-bottom: 1rem;
                color: var(--text-primary);
                border-bottom: 1px solid var(--border-subtle);
                padding-bottom: 0.5rem;
            }

            h3 {
                font-size: 1.375rem;
                font-weight: 600;
                margin-top: 2rem;
                margin-bottom: 0.75rem;
                color: var(--text-primary);
            }

            h4 {
                font-size: 1.125rem;
                font-weight: 600;
                margin-top: 1.5rem;
                margin-bottom: 0.5rem;
                color: var(--text-primary);
            }

            p {
                margin-bottom: 1rem;
                color: var(--text-secondary);
            }

            a {
                color: var(--accent);
                text-decoration: none;
                transition: color 0.2s ease;

                &:hover {
                    text-decoration: underline;
                }
            }

            code:not(pre code) {
                background: var(--surface-elevated);
                padding: 0.2em 0.4em;
                border-radius: 4px;
                font-size: 0.875em;
                font-family: 'JetBrains Mono', 'Fira Code', monospace;
                color: var(--accent);
            }

            pre {
                background: var(--surface-elevated);
                border: 1px solid var(--border-subtle);
                border-radius: 8px;
                padding: 1rem;
                overflow-x: auto;
                margin: 1.5rem 0;

                code {
                    background: none;
                    padding: 0;
                    font-size: 0.875rem;
                    line-height: 1.6;
                    color: var(--text-primary);
                }
            }

            ul, ol {
                margin-bottom: 1rem;
                padding-left: 1.5rem;
                color: var(--text-secondary);

                li {
                    margin-bottom: 0.5rem;
                }
            }

            ul {
                list-style-type: disc;

                ul {
                    list-style-type: circle;
                }
            }

            ol {
                list-style-type: decimal;
            }

            blockquote {
                border-left: 4px solid var(--accent);
                margin: 1.5rem 0;
                padding: 0.5rem 0 0.5rem 1rem;
                background: var(--surface-elevated);
                border-radius: 0 8px 8px 0;

                p {
                    margin: 0;
                    color: var(--text-secondary);
                }
            }

            table {
                width: 100%;
                border-collapse: collapse;
                margin: 1.5rem 0;
                font-size: 0.875rem;

                th, td {
                    padding: 0.75rem 1rem;
                    text-align: left;
                    border: 1px solid var(--border-subtle);
                }

                th {
                    background: var(--surface-elevated);
                    font-weight: 600;
                    color: var(--text-primary);
                }

                td {
                    color: var(--text-secondary);
                }

                tbody tr:nth-child(even) {
                    background: var(--surface-elevated);
                }
            }

            hr {
                border: none;
                border-top: 1px solid var(--border-subtle);
                margin: 2rem 0;
            }

            strong {
                font-weight: 600;
                color: var(--text-primary);
            }

            em {
                font-style: italic;
            }

            .callout {
                padding: 1rem;
                border-radius: 8px;
                margin: 1.5rem 0;
                border-left: 4px solid;

                &.warning {
                    background: rgba(251, 191, 36, 0.1);
                    border-color: #fbbf24;
                }

                &.tip {
                    background: rgba(34, 197, 94, 0.1);
                    border-color: #22c55e;
                }

                &.note {
                    background: rgba(59, 130, 246, 0.1);
                    border-color: #3b82f6;
                }
            }
        }
    `]
})
export class MarkdownComponent implements OnChanges {
    @Input() content: string = '';

    renderedContent: SafeHtml = '';

    private sanitizer = inject(DomSanitizer);
    private platformId = inject(PLATFORM_ID);

    ngOnChanges(changes: SimpleChanges): void {
        if (changes['content']) {
            this.renderedContent = this.sanitizer.bypassSecurityTrustHtml(
                this.parseMarkdown(this.content)
            );
        }
    }

    private parseMarkdown(markdown: string): string {
        if (!markdown) return '';

        let html = markdown;

        // Handle callouts/admonitions (GitHub style)
        html = html.replace(/>\s*\[!(WARNING|TIP|NOTE)\]\s*\n>\s*\*\*([^*]+)\*\*\s*([^\n]+)/gi,
            (_, type, title, content) => `<div class="callout ${type.toLowerCase()}"><strong>${title}</strong> ${content}</div>`);

        // Handle code blocks first (before other processing)
        html = html.replace(/```(\w+)?\n([\s\S]*?)```/g, (_, lang, code) => {
            const escapedCode = this.escapeHtml(code.trim());
            return `<pre><code class="language-${lang || 'text'}">${escapedCode}</code></pre>`;
        });

        // Handle inline code
        html = html.replace(/`([^`]+)`/g, '<code>$1</code>');

        // Handle headers
        html = html.replace(/^#### (.+)$/gm, '<h4>$1</h4>');
        html = html.replace(/^### (.+)$/gm, '<h3>$1</h3>');
        html = html.replace(/^## (.+)$/gm, '<h2>$1</h2>');
        html = html.replace(/^# (.+)$/gm, '<h1>$1</h1>');

        // Handle tables
        html = this.parseTables(html);

        // Handle horizontal rules
        html = html.replace(/^---$/gm, '<hr>');

        // Handle blockquotes
        html = html.replace(/^>\s*(.+)$/gm, '<blockquote><p>$1</p></blockquote>');
        // Merge consecutive blockquotes
        html = html.replace(/<\/blockquote>\n<blockquote>/g, '\n');

        // Handle bold and italic
        html = html.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
        html = html.replace(/\*([^*]+)\*/g, '<em>$1</em>');

        // Handle links
        html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2">$1</a>');

        // Handle unordered lists
        html = this.parseUnorderedLists(html);

        // Handle ordered lists
        html = this.parseOrderedLists(html);

        // Handle paragraphs - wrap remaining text
        html = this.wrapParagraphs(html);

        return html;
    }

    private escapeHtml(text: string): string {
        const div = isPlatformBrowser(this.platformId) ? document.createElement('div') : null;
        if (div) {
            div.textContent = text;
            return div.innerHTML;
        }
        // SSR fallback
        return text
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#039;');
    }

    private parseTables(html: string): string {
        const tableRegex = /\|(.+)\|\n\|[-|\s]+\|\n((?:\|.+\|\n?)+)/g;

        return html.replace(tableRegex, (_, headerRow, bodyRows) => {
            const headers = headerRow.split('|').filter((h: string) => h.trim());
            const rows = bodyRows.trim().split('\n');

            let table = '<table><thead><tr>';
            headers.forEach((header: string) => {
                table += `<th>${header.trim()}</th>`;
            });
            table += '</tr></thead><tbody>';

            rows.forEach((row: string) => {
                const cells = row.split('|').filter((c: string) => c.trim());
                table += '<tr>';
                cells.forEach((cell: string) => {
                    table += `<td>${cell.trim()}</td>`;
                });
                table += '</tr>';
            });

            table += '</tbody></table>';
            return table;
        });
    }

    private parseUnorderedLists(html: string): string {
        const lines = html.split('\n');
        let result: string[] = [];
        let inList = false;
        let listItems: string[] = [];

        for (const line of lines) {
            const match = line.match(/^(\s*)[-*]\s+(.+)$/);
            if (match) {
                if (!inList) {
                    inList = true;
                    listItems = [];
                }
                listItems.push(`<li>${match[2]}</li>`);
            } else {
                if (inList) {
                    result.push(`<ul>${listItems.join('')}</ul>`);
                    inList = false;
                    listItems = [];
                }
                result.push(line);
            }
        }

        if (inList) {
            result.push(`<ul>${listItems.join('')}</ul>`);
        }

        return result.join('\n');
    }

    private parseOrderedLists(html: string): string {
        const lines = html.split('\n');
        let result: string[] = [];
        let inList = false;
        let listItems: string[] = [];

        for (const line of lines) {
            const match = line.match(/^\d+\.\s+(.+)$/);
            if (match) {
                if (!inList) {
                    inList = true;
                    listItems = [];
                }
                listItems.push(`<li>${match[1]}</li>`);
            } else {
                if (inList) {
                    result.push(`<ol>${listItems.join('')}</ol>`);
                    inList = false;
                    listItems = [];
                }
                result.push(line);
            }
        }

        if (inList) {
            result.push(`<ol>${listItems.join('')}</ol>`);
        }

        return result.join('\n');
    }

    private wrapParagraphs(html: string): string {
        const lines = html.split('\n');
        let result: string[] = [];
        let paragraph: string[] = [];

        const isBlockElement = (line: string) => {
            return line.startsWith('<h') ||
                   line.startsWith('<ul') ||
                   line.startsWith('<ol') ||
                   line.startsWith('<pre') ||
                   line.startsWith('<table') ||
                   line.startsWith('<blockquote') ||
                   line.startsWith('<hr') ||
                   line.startsWith('<div') ||
                   line.trim() === '';
        };

        for (const line of lines) {
            if (isBlockElement(line)) {
                if (paragraph.length > 0) {
                    result.push(`<p>${paragraph.join(' ')}</p>`);
                    paragraph = [];
                }
                if (line.trim() !== '') {
                    result.push(line);
                }
            } else {
                paragraph.push(line);
            }
        }

        if (paragraph.length > 0) {
            result.push(`<p>${paragraph.join(' ')}</p>`);
        }

        return result.join('\n');
    }
}
