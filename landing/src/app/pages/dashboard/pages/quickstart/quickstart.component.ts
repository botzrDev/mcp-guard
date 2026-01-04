import { Component, ChangeDetectionStrategy, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

@Component({
    selector: 'app-quickstart',
    standalone: true,
    imports: [CommonModule, RouterModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="quickstart-page">
            <div class="page-header">
                <h1>Quickstart Guide</h1>
                <p class="page-subtitle">Get your MCP Guard up and running in minutes.</p>
            </div>

            <div class="steps-container">
                <div class="step-card">
                    <div class="step-number">1</div>
                    <div class="step-content">
                        <h2>Install MCP Guard</h2>
                        <p>You can run MCP Guard using Docker or as a binary.</p>
                        
                        <div class="tabs">
                            <button 
                                class="tab-btn" 
                                [class.active]="installMethod() === 'docker'"
                                (click)="installMethod.set('docker')"
                            >
                                Docker
                            </button>
                            <button 
                                class="tab-btn" 
                                [class.active]="installMethod() === 'binary'"
                                (click)="installMethod.set('binary')"
                            >
                                Binary
                            </button>
                        </div>

                        @if (installMethod() === 'docker') {
                            <div class="code-block">
                                <code>docker pull ghcr.io/botzrdev/mcp-guard:latest</code>
                                <button class="copy-btn" (click)="copyCode('docker pull ghcr.io/botzrdev/mcp-guard:latest')">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                        <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                    </svg>
                                </button>
                            </div>
                        } @else {
                            <div class="code-block">
                                <code>curl -fsSL https://mcp-guard.com/install.sh | sh</code>
                                <button class="copy-btn" (click)="copyCode('curl -fsSL https://mcp-guard.com/install.sh | sh')">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                        <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                    </svg>
                                </button>
                            </div>
                        }
                    </div>
                </div>

                <div class="step-card">
                    <div class="step-number">2</div>
                    <div class="step-content">
                        <h2>Get Your License Key</h2>
                        <p>
                            You need a license key to start the server. 
                            You can find it in the <a routerLink="/dashboard/license">License</a> page.
                        </p>
                    </div>
                </div>

                <div class="step-card">
                    <div class="step-number">3</div>
                    <div class="step-content">
                        <h2>Create an API Key</h2>
                        <p>
                            Create an API key for your clients to authenticate.
                            Go to <a routerLink="/dashboard/api-keys">API Keys</a> to generate one.
                        </p>
                    </div>
                </div>

                <div class="step-card">
                    <div class="step-number">4</div>
                    <div class="step-content">
                        <h2>Run the Server</h2>
                        <p>Start MCP Guard with your license key and configuration.</p>
                        
                        <div class="code-block">
                            <code>docker run -p 8080:8080 -e MCP_GUARD_LICENSE_KEY=your_license_key mcp-guard</code>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    `,
    styles: [`
        .quickstart-page {
            max-width: 800px;
            margin: 0 auto;
        }

        .page-header {
            margin-bottom: var(--space-8);

            h1 {
                font-family: var(--font-display);
                font-size: var(--text-3xl);
                font-weight: var(--weight-bold);
                margin-bottom: var(--space-2);
            }
        }

        .page-subtitle {
            color: var(--text-muted);
            font-size: var(--text-base);
        }

        .steps-container {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
        }

        .step-card {
            display: flex;
            gap: var(--space-5);
            padding: var(--space-6);
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
        }

        .step-number {
            width: var(--space-8);
            height: var(--space-8);
            background: var(--bg-elevated);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-full);
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: var(--font-mono);
            font-weight: var(--weight-bold);
            color: var(--accent-cyan);
            flex-shrink: 0;
        }

        .step-content {
            flex: 1;

            h2 {
                font-size: var(--text-lg);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-2);
            }

            p {
                color: var(--text-secondary);
                margin-bottom: var(--space-4);
                line-height: var(--leading-relaxed);

                a {
                    color: var(--accent-cyan);
                    text-decoration: none;
                    &:hover { text-decoration: underline; }
                }
            }
        }

        .tabs {
            display: flex;
            gap: var(--space-2);
            margin-bottom: var(--space-4);
        }

        .tab-btn {
            padding: var(--space-2) var(--space-4);
            background: var(--bg-primary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);
            color: var(--text-secondary);
            font-size: var(--text-sm);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            &.active {
                background: var(--bg-elevated);
                color: var(--text-primary);
                border-color: var(--accent-cyan);
            }
        }

        .code-block {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-3) var(--space-4);
            background: var(--bg-primary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);

            code {
                flex: 1;
                font-family: var(--font-mono);
                font-size: var(--text-sm);
                color: var(--accent-cyan);
                overflow-x: auto;
                white-space: nowrap;
            }
        }

        .copy-btn {
            width: var(--space-8);
            height: var(--space-8);
            display: flex;
            align-items: center;
            justify-content: center;
            background: transparent;
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-md);
            color: var(--text-muted);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }

            &:hover {
                background: var(--bg-elevated);
                color: var(--text-primary);
            }
        }
    `]
})
export class QuickstartComponent {
    installMethod = signal<'docker' | 'binary'>('docker');

    copyCode(code: string): void {
        navigator.clipboard.writeText(code);
    }
}
