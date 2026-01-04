import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
    selector: 'app-health',
    standalone: true,
    imports: [CommonModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="health-page">
            <div class="page-header">
                <h1>System Health</h1>
                <div class="last-updated">
                    Updated: just now
                    <button class="refresh-btn" (click)="refreshHealth()">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M23 4v6h-6M1 20v-6h6"/>
                            <path d="M3.51 9a9 9 0 0114.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0020.49 15"/>
                        </svg>
                    </button>
                </div>
            </div>

            <div class="status-overview">
                <div class="main-status healthy">
                    <div class="status-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M22 11.08V12a10 10 0 11-5.93-9.14"/>
                            <polyline points="22 4 12 14.01 9 11.01"/>
                        </svg>
                    </div>
                    <div class="status-text">
                        <h2>All Systems Operational</h2>
                        <p>MCP Guard is running normally.</p>
                    </div>
                </div>
                
                <div class="metrics-grid">
                    <div class="metric-card">
                        <span class="metric-label">Uptime</span>
                        <span class="metric-value">99.99%</span>
                        <span class="metric-sub">Last 30 days</span>
                    </div>
                    <div class="metric-card">
                        <span class="metric-label">Current Load</span>
                        <span class="metric-value">24%</span>
                        <span class="metric-sub">CPU Usage</span>
                    </div>
                    <div class="metric-card">
                        <span class="metric-label">Memory</span>
                        <span class="metric-value">1.2GB</span>
                        <span class="metric-sub">of 4.0GB</span>
                    </div>
                </div>
            </div>

            <div class="components-grid">
                @for (component of components(); track component.name) {
                    <div class="component-card">
                        <div class="component-header">
                            <span class="component-name">{{ component.name }}</span>
                            <span class="component-status" [class]="component.status">
                                {{ component.status }}
                            </span>
                        </div>
                        <div class="component-details">
                            <div class="detail-row">
                                <span class="label">Latency</span>
                                <span class="value">{{ component.latency }}ms</span>
                            </div>
                            <div class="detail-row">
                                <span class="label">Uptime</span>
                                <span class="value">{{ component.uptime }}</span>
                            </div>
                        </div>
                        <div class="latency-sparkline">
                            @for (bar of component.history; track $index) {
                                <div class="bar" [style.height.%]="bar"></div>
                            }
                        </div>
                    </div>
                }
            </div>
        </div>
    `,
    styles: [`
        .health-page {
            max-width: 1000px;
            margin: 0 auto;
        }

        .page-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: var(--space-8);

            h1 {
                font-family: var(--font-display);
                font-size: var(--text-3xl);
                font-weight: var(--weight-bold);
            }
        }

        .last-updated {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .refresh-btn {
            background: transparent;
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-md);
            width: var(--space-8);
            height: var(--space-8);
            display: flex;
            align-items: center;
            justify-content: center;
            color: var(--text-secondary);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }

            &:hover {
                background: var(--bg-elevated);
                color: var(--text-primary);
                transform: rotate(180deg);
            }
        }

        .status-overview {
            margin-bottom: var(--space-8);
            display: grid;
            grid-template-columns: 1.5fr 1fr;
            gap: var(--space-6);

            @media (max-width: 768px) {
                grid-template-columns: 1fr;
            }
        }

        .main-status {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-2xl);
            padding: var(--space-6);
            display: flex;
            align-items: center;
            gap: var(--space-5);

            &.healthy {
                background: linear-gradient(135deg, rgba(34, 197, 94, 0.1), rgba(34, 197, 94, 0.05));
                border-color: rgba(34, 197, 94, 0.2);

                .status-icon {
                    background: rgba(34, 197, 94, 0.2);
                    color: var(--accent-green);
                }
            }
        }

        .status-icon {
            width: var(--space-12);
            height: var(--space-12);
            border-radius: var(--radius-full);
            display: flex;
            align-items: center;
            justify-content: center;

            svg {
                width: var(--icon-md);
                height: var(--icon-md);
            }
        }

        .status-text {
            h2 {
                font-size: var(--text-xl);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-1);
            }

            p {
                color: var(--text-muted);
            }
        }

        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: var(--space-4);
        }

        .metric-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-4);
            display: flex;
            flex-direction: column;
            text-align: center;
        }

        .metric-label {
            font-size: var(--text-xs);
            color: var(--text-muted);
            margin-bottom: var(--space-2);
        }

        .metric-value {
            font-size: var(--text-xl);
            font-weight: var(--weight-bold);
            margin-bottom: var(--space-1);
        }

        .metric-sub {
            font-size: var(--text-2xs);
            color: var(--text-dim);
        }

        .components-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: var(--space-4);
        }

        .component-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-5);
        }

        .component-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: var(--space-4);
        }

        .component-name {
            font-weight: var(--weight-semibold);
        }

        .component-status {
            padding: var(--space-1) var(--space-2);
            border-radius: var(--radius-full);
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);
            text-transform: capitalize;

            &.operational {
                background: rgba(34, 197, 94, 0.1);
                color: var(--accent-green);
            }

            &.degraded {
                background: rgba(249, 115, 22, 0.1);
                color: var(--accent-orange);
            }

            &.outage {
                background: rgba(239, 68, 68, 0.1);
                color: var(--accent-red);
            }
        }

        .component-details {
            display: grid;
            gap: var(--space-2);
            margin-bottom: var(--space-4);
        }

        .detail-row {
            display: flex;
            justify-content: space-between;
            font-size: var(--text-sm);

            .label { color: var(--text-muted); }
            .value { font-family: var(--font-mono); }
        }

        .latency-sparkline {
            display: flex;
            align-items: flex-end;
            gap: 4px;
            height: 32px;
            padding-top: var(--space-2);
            border-top: 1px solid var(--border-subtle);

            .bar {
                flex: 1;
                background: var(--bg-elevated);
                border-radius: 1px;
                min-height: 2px;
            }
        }
    `]
})
export class HealthComponent {
    lastUpdated = signal(new Date());

    components = signal([
        {
            name: 'API Gateway',
            status: 'operational',
            latency: 45,
            uptime: '99.99%',
            history: [30, 40, 35, 50, 45, 40, 35, 30, 45, 50, 40, 35]
        },
        {
            name: 'Database',
            status: 'operational',
            latency: 12,
            uptime: '99.95%',
            history: [20, 25, 20, 30, 25, 20, 15, 20, 25, 30, 25, 20]
        },
        {
            name: 'Auth Service',
            status: 'operational',
            latency: 85,
            uptime: '99.90%',
            history: [60, 70, 65, 80, 75, 70, 65, 60, 75, 80, 70, 65]
        },
        {
            name: 'Redis Cache',
            status: 'operational',
            latency: 2,
            uptime: '99.99%',
            history: [10, 15, 10, 20, 15, 10, 5, 10, 15, 20, 15, 10]
        }
    ]);

    refreshHealth() {
        this.lastUpdated.set(new Date());
        // In a real app, this would fetch new data
    }
}
