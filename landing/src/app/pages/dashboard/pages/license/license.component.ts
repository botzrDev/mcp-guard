import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

@Component({
    selector: 'app-license',
    standalone: true,
    imports: [CommonModule, RouterModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="license-page">
            <div class="page-header">
                <h1>License</h1>
                <p class="page-subtitle">Manage your MCP Guard license and subscription.</p>
            </div>

            <div class="license-card">
                <div class="license-header">
                    <div class="tier-badge" [class]="currentTier()">
                        {{ currentTier() | titlecase }}
                    </div>
                    <span class="status-badge active">Active</span>
                </div>

                <div class="license-key-section">
                    <label class="section-label">License Key</label>
                    <div class="key-display">
                        <code class="key-value">
                            @if (showKey()) {
                                {{ licenseKey() }}
                            } @else {
                                {{ maskedKey() }}
                            }
                        </code>
                        <div class="key-actions">
                            <button class="icon-btn" (click)="toggleShowKey()" [title]="showKey() ? 'Hide' : 'Show'">
                                @if (showKey()) {
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19m-6.72-1.07a3 3 0 11-4.24-4.24"/>
                                        <line x1="1" y1="1" x2="23" y2="23"/>
                                    </svg>
                                } @else {
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                                        <circle cx="12" cy="12" r="3"/>
                                    </svg>
                                }
                            </button>
                            <button class="icon-btn" (click)="copyKey()" title="Copy">
                                @if (copied()) {
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <polyline points="20 6 9 17 4 12"/>
                                    </svg>
                                } @else {
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                        <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                    </svg>
                                }
                            </button>
                        </div>
                    </div>
                    @if (copied()) {
                        <span class="copy-feedback">Copied to clipboard!</span>
                    }
                </div>

                <div class="license-details">
                    <div class="detail-row">
                        <span class="detail-label">Plan</span>
                        <span class="detail-value">{{ currentTier() | titlecase }}</span>
                    </div>
                    <div class="detail-row">
                        <span class="detail-label">Status</span>
                        <span class="detail-value status-active">Active</span>
                    </div>
                    <div class="detail-row">
                        <span class="detail-label">Billing Period</span>
                        <span class="detail-value">Monthly</span>
                    </div>
                    <div class="detail-row">
                        <span class="detail-label">Next Billing Date</span>
                        <span class="detail-value">{{ nextBillingDate() }}</span>
                    </div>
                </div>
            </div>

            <div class="usage-section">
                <h2>How to Use Your License</h2>
                <div class="usage-instructions">
                    <div class="instruction-block">
                        <h3>Environment Variable</h3>
                        <p>Add to your shell profile or .env file:</p>
                        <div class="code-block">
                            <code>export MCP_GUARD_LICENSE_KEY="{{ licenseKey() }}"</code>
                            <button class="copy-code-btn" (click)="copyEnvCommand()">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                    <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                </svg>
                            </button>
                        </div>
                    </div>

                    <div class="instruction-block">
                        <h3>Docker</h3>
                        <p>Pass as environment variable:</p>
                        <div class="code-block">
                            <code>docker run -e MCP_GUARD_LICENSE_KEY=... mcp-guard</code>
                            <button class="copy-code-btn" (click)="copyDockerCommand()">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                    <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                </svg>
                            </button>
                        </div>
                    </div>

                    <div class="instruction-block">
                        <h3>systemd Service</h3>
                        <p>Add to your service file:</p>
                        <div class="code-block">
                            <code>Environment=MCP_GUARD_LICENSE_KEY={{ licenseKey() }}</code>
                            <button class="copy-code-btn" (click)="copySystemdCommand()">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                    <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            @if (currentTier() !== 'enterprise') {
                <div class="upgrade-section">
                    <h2>Upgrade Your Plan</h2>
                    <div class="upgrade-cards">
                        @if (currentTier() === 'free') {
                            <div class="upgrade-card pro">
                                <div class="upgrade-header">
                                    <span class="upgrade-name">Pro</span>
                                    <span class="upgrade-price">$12<span>/month</span></span>
                                </div>
                                <ul class="upgrade-features">
                                    <li>OAuth 2.1 + PKCE authentication</li>
                                    <li>JWT JWKS (RS256/ES256)</li>
                                    <li>HTTP & SSE transports</li>
                                    <li>Per-identity rate limiting</li>
                                    <li>Email support (48h SLA)</li>
                                </ul>
                                <a routerLink="/signup" [queryParams]="{plan: 'pro'}" class="upgrade-btn">
                                    Upgrade to Pro
                                </a>
                            </div>
                        }
                        <div class="upgrade-card enterprise">
                            <div class="upgrade-header">
                                <span class="upgrade-name">Enterprise</span>
                                <span class="upgrade-price">$29<span> + $8/seat</span></span>
                            </div>
                            <ul class="upgrade-features">
                                <li>Everything in Pro, plus:</li>
                                <li>mTLS client certificates</li>
                                <li>Multi-server routing</li>
                                <li>OpenTelemetry tracing</li>
                                <li>SIEM log shipping</li>
                                <li>Priority support (4h SLA)</li>
                            </ul>
                            <a routerLink="/contact" [queryParams]="{plan: 'enterprise'}" class="upgrade-btn">
                                Contact Sales
                            </a>
                        </div>
                    </div>
                </div>
            }
        </div>
    `,
    styles: [`
        .license-page {
            max-width: 900px;
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

        .license-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-2xl);
            padding: var(--space-6);
            margin-bottom: var(--space-8);
        }

        .license-header {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            margin-bottom: var(--space-6);
        }

        .tier-badge {
            padding: var(--space-1-5) var(--space-3);
            border-radius: var(--radius-full);
            font-size: var(--text-sm);
            font-weight: var(--weight-bold);

            &.free {
                background: rgba(100, 116, 139, 0.1);
                color: var(--text-secondary);
            }

            &.pro {
                background: linear-gradient(135deg, rgba(249, 115, 22, 0.1), rgba(239, 68, 68, 0.1));
                color: var(--accent-orange);
            }

            &.enterprise {
                background: linear-gradient(135deg, rgba(139, 92, 246, 0.1), rgba(59, 130, 246, 0.1));
                color: var(--accent-purple);
            }
        }

        .status-badge {
            padding: var(--space-1) var(--space-2);
            border-radius: var(--radius-sm);
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);

            &.active {
                background: rgba(34, 197, 94, 0.1);
                color: var(--accent-green);
            }
        }

        .license-key-section {
            margin-bottom: var(--space-6);
        }

        .section-label {
            display: block;
            font-size: var(--text-sm);
            font-weight: var(--weight-medium);
            color: var(--text-muted);
            margin-bottom: var(--space-2);
        }

        .key-display {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-3) var(--space-4);
            background: var(--bg-primary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);
        }

        .key-value {
            flex: 1;
            font-family: var(--font-mono);
            font-size: var(--text-sm);
            color: var(--accent-cyan);
            word-break: break-all;
        }

        .key-actions {
            display: flex;
            gap: var(--space-1);
        }

        .icon-btn {
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
                border-color: var(--border-accent);
            }
        }

        .copy-feedback {
            display: block;
            margin-top: var(--space-2);
            font-size: var(--text-xs);
            color: var(--accent-green);
        }

        .license-details {
            display: grid;
            gap: var(--space-3);
        }

        .detail-row {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: var(--space-3) 0;
            border-bottom: 1px solid var(--border-subtle);

            &:last-child {
                border-bottom: none;
            }
        }

        .detail-label {
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .detail-value {
            font-size: var(--text-sm);
            font-weight: var(--weight-medium);

            &.status-active {
                color: var(--accent-green);
            }
        }

        .usage-section {
            margin-bottom: var(--space-8);

            h2 {
                font-size: var(--text-xl);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-4);
            }
        }

        .usage-instructions {
            display: grid;
            gap: var(--space-4);
        }

        .instruction-block {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-5);

            h3 {
                font-size: var(--text-base);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-2);
            }

            p {
                font-size: var(--text-sm);
                color: var(--text-muted);
                margin-bottom: var(--space-3);
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
                font-size: var(--text-xs);
                color: var(--accent-cyan);
                overflow-x: auto;
                white-space: nowrap;
            }
        }

        .copy-code-btn {
            width: var(--space-8);
            height: var(--space-8);
            display: flex;
            align-items: center;
            justify-content: center;
            background: transparent;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            border-radius: var(--radius-md);
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

        .upgrade-section {
            h2 {
                font-size: var(--text-xl);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-4);
            }
        }

        .upgrade-cards {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: var(--space-4);
        }

        .upgrade-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-6);

            &.pro {
                border-color: rgba(249, 115, 22, 0.3);
            }

            &.enterprise {
                border-color: rgba(139, 92, 246, 0.3);
            }
        }

        .upgrade-header {
            display: flex;
            justify-content: space-between;
            align-items: baseline;
            margin-bottom: var(--space-4);
        }

        .upgrade-name {
            font-size: var(--text-lg);
            font-weight: var(--weight-semibold);
        }

        .upgrade-price {
            font-size: var(--text-2xl);
            font-weight: var(--weight-bold);

            span {
                font-size: var(--text-sm);
                font-weight: var(--weight-normal);
                color: var(--text-muted);
            }
        }

        .upgrade-features {
            list-style: none;
            margin-bottom: var(--space-5);

            li {
                display: flex;
                align-items: center;
                gap: var(--space-2);
                padding: var(--space-2) 0;
                font-size: var(--text-sm);
                color: var(--text-secondary);

                &::before {
                    content: '✓';
                    color: var(--accent-cyan);
                    font-weight: var(--weight-bold);
                }
            }
        }

        .upgrade-btn {
            display: block;
            width: 100%;
            padding: var(--space-3) var(--space-4);
            background: var(--bg-elevated);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);
            text-align: center;
            text-decoration: none;
            color: var(--text-primary);
            font-size: var(--text-sm);
            font-weight: var(--weight-semibold);
            transition: all var(--duration-fast) var(--ease-out);

            &:hover {
                background: var(--text-primary);
                color: var(--bg-primary);
            }
        }
    `]
})
export class LicenseComponent {
    showKey = signal(false);
    copied = signal(false);
    currentTier = signal<'free' | 'pro' | 'enterprise'>('free');
    licenseKey = signal('pro_demo_key_abc123xyz789');
    nextBillingDate = signal('January 15, 2026');

    maskedKey(): string {
        const key = this.licenseKey();
        if (key.length <= 12) return key;
        return key.substring(0, 8) + '...' + key.substring(key.length - 4);
    }

    toggleShowKey(): void {
        this.showKey.update(v => !v);
    }

    copyKey(): void {
        navigator.clipboard.writeText(this.licenseKey());
        this.copied.set(true);
        setTimeout(() => this.copied.set(false), 2000);
    }

    copyEnvCommand(): void {
        navigator.clipboard.writeText(`export MCP_GUARD_LICENSE_KEY="${this.licenseKey()}"`);
    }

    copyDockerCommand(): void {
        navigator.clipboard.writeText(`docker run -e MCP_GUARD_LICENSE_KEY="${this.licenseKey()}" mcp-guard`);
    }

    copySystemdCommand(): void {
        navigator.clipboard.writeText(`Environment=MCP_GUARD_LICENSE_KEY=${this.licenseKey()}`);
    }
}
