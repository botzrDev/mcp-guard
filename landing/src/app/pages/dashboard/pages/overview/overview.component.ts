import { Component, inject, signal, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { AuthService } from '../../../../core/auth';

interface StatCard {
    label: string;
    value: string | number;
    change?: string;
    changeType?: 'positive' | 'negative' | 'neutral';
    icon: string;
}

@Component({
    selector: 'app-overview',
    standalone: true,
    imports: [CommonModule, RouterModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="overview">
            <div class="page-header">
                <h1>Welcome back, {{ (authService.user()?.name?.split(' ')?.[0]) ?? 'User' }}</h1>
                <p class="page-subtitle">Here's what's happening with your MCP Guard setup.</p>
            </div>

            <div class="stats-grid">
                @for (stat of stats(); track stat.label) {
                    <div class="stat-card">
                        <div class="stat-icon" [innerHTML]="stat.icon"></div>
                        <div class="stat-content">
                            <span class="stat-label">{{ stat.label }}</span>
                            <span class="stat-value">{{ stat.value }}</span>
                            @if (stat.change) {
                                <span class="stat-change" [class]="stat.changeType">
                                    {{ stat.change }}
                                </span>
                            }
                        </div>
                    </div>
                }
            </div>

            <div class="quick-actions">
                <h2>Quick Actions</h2>
                <div class="actions-grid">
                    <a routerLink="/dashboard/api-keys" class="action-card">
                        <div class="action-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                                <path d="M7 11V7a5 5 0 0110 0v4"/>
                            </svg>
                        </div>
                        <div class="action-content">
                            <span class="action-title">Create API Key</span>
                            <span class="action-desc">Generate a new key for your MCP server</span>
                        </div>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="action-arrow">
                            <path d="M5 12h14M12 5l7 7-7 7"/>
                        </svg>
                    </a>

                    <a routerLink="/dashboard/quickstart" class="action-card">
                        <div class="action-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 00-2.91-.09z"/>
                                <path d="M12 15l-3-3a22 22 0 012-3.95A12.88 12.88 0 0122 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 01-4 2z"/>
                            </svg>
                        </div>
                        <div class="action-content">
                            <span class="action-title">Quick Start Guide</span>
                            <span class="action-desc">Get up and running in 5 minutes</span>
                        </div>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="action-arrow">
                            <path d="M5 12h14M12 5l7 7-7 7"/>
                        </svg>
                    </a>

                    <a routerLink="/docs" class="action-card">
                        <div class="action-icon">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/>
                                <polyline points="14 2 14 8 20 8"/>
                                <line x1="16" y1="13" x2="8" y2="13"/>
                                <line x1="16" y1="17" x2="8" y2="17"/>
                            </svg>
                        </div>
                        <div class="action-content">
                            <span class="action-title">Documentation</span>
                            <span class="action-desc">Learn about all features and configuration</span>
                        </div>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="action-arrow">
                            <path d="M5 12h14M12 5l7 7-7 7"/>
                        </svg>
                    </a>
                </div>
            </div>

            <div class="license-banner" [class]="licenseStatus()">
                <div class="banner-content">
                    <div class="banner-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                        </svg>
                    </div>
                    <div class="banner-text">
                        <span class="banner-title">{{ licenseTier() }} Plan</span>
                        <span class="banner-desc">{{ licenseMessage() }}</span>
                    </div>
                </div>
                @if (licenseStatus() === 'free') {
                    <a routerLink="/dashboard/license" class="banner-action">
                        Upgrade to Pro
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M5 12h14M12 5l7 7-7 7"/>
                        </svg>
                    </a>
                }
            </div>
        </div>
    `,
    styles: [`
        .overview {
            max-width: 1200px;
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

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
            gap: var(--space-4);
            margin-bottom: var(--space-8);
        }

        .stat-card {
            display: flex;
            align-items: flex-start;
            gap: var(--space-4);
            padding: var(--space-5);
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
        }

        .stat-icon {
            width: var(--space-12);
            height: var(--space-12);
            background: rgba(78, 205, 196, 0.1);
            border-radius: var(--radius-lg);
            display: flex;
            align-items: center;
            justify-content: center;
            color: var(--accent-cyan);
            flex-shrink: 0;

            :deep(svg) {
                width: var(--icon-lg);
                height: var(--icon-lg);
            }
        }

        .stat-content {
            display: flex;
            flex-direction: column;
            gap: var(--space-1);
        }

        .stat-label {
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .stat-value {
            font-size: var(--text-2xl);
            font-weight: var(--weight-bold);
        }

        .stat-change {
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);

            &.positive {
                color: var(--accent-green);
            }

            &.negative {
                color: var(--accent-red);
            }

            &.neutral {
                color: var(--text-muted);
            }
        }

        .quick-actions {
            margin-bottom: var(--space-8);

            h2 {
                font-size: var(--text-xl);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-4);
            }
        }

        .actions-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: var(--space-4);
        }

        .action-card {
            display: flex;
            align-items: center;
            gap: var(--space-4);
            padding: var(--space-4);
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            text-decoration: none;
            color: inherit;
            transition: all var(--duration-fast) var(--ease-out);

            &:hover {
                background: var(--bg-elevated);
                border-color: var(--border-accent);

                .action-arrow {
                    transform: translateX(4px);
                }
            }
        }

        .action-icon {
            width: var(--space-10);
            height: var(--space-10);
            background: var(--bg-elevated);
            border-radius: var(--radius-lg);
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;

            svg {
                width: var(--icon-md);
                height: var(--icon-md);
                color: var(--accent-cyan);
            }
        }

        .action-content {
            flex: 1;
            display: flex;
            flex-direction: column;
            gap: var(--space-0-5);
        }

        .action-title {
            font-size: var(--text-sm);
            font-weight: var(--weight-semibold);
        }

        .action-desc {
            font-size: var(--text-xs);
            color: var(--text-muted);
        }

        .action-arrow {
            width: var(--icon-sm);
            height: var(--icon-sm);
            color: var(--text-muted);
            transition: transform var(--duration-fast) var(--ease-out);
        }

        .license-banner {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: var(--space-5);
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);

            &.free {
                background: linear-gradient(90deg, rgba(78, 205, 196, 0.05) 0%, rgba(59, 130, 246, 0.05) 100%);
                border-color: rgba(78, 205, 196, 0.2);
            }

            &.pro {
                background: linear-gradient(90deg, rgba(249, 115, 22, 0.05) 0%, rgba(239, 68, 68, 0.05) 100%);
                border-color: rgba(249, 115, 22, 0.2);
            }

            &.enterprise {
                background: linear-gradient(90deg, rgba(139, 92, 246, 0.05) 0%, rgba(59, 130, 246, 0.05) 100%);
                border-color: rgba(139, 92, 246, 0.2);
            }

            @media (max-width: 640px) {
                flex-direction: column;
                gap: var(--space-4);
                text-align: center;
            }
        }

        .banner-content {
            display: flex;
            align-items: center;
            gap: var(--space-4);

            @media (max-width: 640px) {
                flex-direction: column;
            }
        }

        .banner-icon {
            width: var(--space-12);
            height: var(--space-12);
            background: var(--bg-elevated);
            border-radius: var(--radius-lg);
            display: flex;
            align-items: center;
            justify-content: center;

            svg {
                width: var(--icon-lg);
                height: var(--icon-lg);
                color: var(--accent-cyan);
            }
        }

        .banner-text {
            display: flex;
            flex-direction: column;
            gap: var(--space-0-5);
        }

        .banner-title {
            font-size: var(--text-lg);
            font-weight: var(--weight-semibold);
        }

        .banner-desc {
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .banner-action {
            display: flex;
            align-items: center;
            gap: var(--space-2);
            padding: var(--space-2-5) var(--space-4);
            background: var(--gradient-brand);
            color: var(--bg-primary);
            text-decoration: none;
            font-size: var(--text-sm);
            font-weight: var(--weight-semibold);
            border-radius: var(--radius-lg);
            transition: all var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }

            &:hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow-lg);
            }
        }
    `]
})
export class OverviewComponent implements OnInit {
    authService = inject(AuthService);

    stats = signal<StatCard[]>([
        {
            label: 'API Calls This Month',
            value: '1,234',
            change: '+12% from last month',
            changeType: 'positive',
            icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>'
        },
        {
            label: 'Active API Keys',
            value: '3',
            icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0110 0v4"/></svg>'
        },
        {
            label: 'Success Rate',
            value: '99.8%',
            change: '+0.2% from last month',
            changeType: 'positive',
            icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 11-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>'
        },
        {
            label: 'Avg Latency',
            value: '45ms',
            change: '-5ms from last month',
            changeType: 'positive',
            icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>'
        }
    ]);

    licenseTier = signal('Free');
    licenseStatus = signal<'free' | 'pro' | 'enterprise'>('free');
    licenseMessage = signal('Upgrade to Pro for OAuth 2.1, HTTP/SSE transports, and more.');

    ngOnInit(): void {
        // In production, fetch actual license info from API
    }
}
