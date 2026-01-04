import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
    selector: 'app-analytics',
    standalone: true,
    imports: [CommonModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="analytics-page">
            <div class="page-header">
                <h1>System Analytics</h1>
                <div class="header-actions">
                    <div class="period-selector">
                        <button class="period-btn active">24h</button>
                        <button class="period-btn">7d</button>
                        <button class="period-btn">30d</button>
                    </div>
                </div>
            </div>

            <div class="stats-grid">
                <div class="stat-card">
                    <span class="stat-label">Total Requests</span>
                    <span class="stat-value">2.4M</span>
                    <span class="stat-change positive">+12% vs last period</span>
                </div>
                <div class="stat-card">
                    <span class="stat-label">Active Users</span>
                    <span class="stat-value">843</span>
                    <span class="stat-change positive">+5% vs last period</span>
                </div>
                <div class="stat-card">
                    <span class="stat-label">Avg Latency</span>
                    <span class="stat-value">42ms</span>
                    <span class="stat-change negative">+2ms vs last period</span>
                </div>
                <div class="stat-card">
                    <span class="stat-label">Error Rate</span>
                    <span class="stat-value">0.02%</span>
                    <span class="stat-change positive">-0.01% vs last period</span>
                </div>
            </div>

            <div class="charts-section">
                <div class="chart-card main">
                    <h2>Traffic Overview</h2>
                    <div class="chart-box">
                        <div class="bars">
                            @for (bar of trafficData(); track bar.label) {
                                <div class="bar-col">
                                    <div class="bar" [style.height.%]="bar.value"></div>
                                    <span class="bar-label">{{ bar.label }}</span>
                                </div>
                            }
                        </div>
                    </div>
                </div>
            </div>

            <div class="secondary-charts">
                <div class="chart-card">
                    <h2>Top Tools</h2>
                    <div class="list-chart">
                        @for (tool of topTools(); track tool.name) {
                            <div class="list-item">
                                <span class="item-name">{{ tool.name }}</span>
                                <div class="item-bar-bg">
                                    <div class="item-bar" [style.width.%]="tool.percentage"></div>
                                </div>
                                <span class="item-value">{{ tool.count | number }}</span>
                            </div>
                        }
                    </div>
                </div>

                <div class="chart-card">
                    <h2>Errors by Type</h2>
                    <div class="list-chart">
                        @for (error of errorsByType(); track error.type) {
                            <div class="list-item">
                                <span class="item-name">{{ error.type }}</span>
                                <div class="item-bar-bg">
                                    <div class="item-bar error" [style.width.%]="error.percentage"></div>
                                </div>
                                <span class="item-value">{{ error.count }}</span>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
    `,
    styles: [`
        .analytics-page {
            max-width: 1200px;
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

        .period-selector {
            display: flex;
            gap: var(--space-2);
            padding: var(--space-1);
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);
        }

        .period-btn {
            padding: var(--space-1-5) var(--space-3);
            background: transparent;
            border: none;
            border-radius: var(--radius-md);
            color: var(--text-secondary);
            font-size: var(--text-sm);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            &.active {
                background: var(--bg-elevated);
                color: var(--text-primary);
                font-weight: var(--weight-medium);
            }

            &:hover:not(.active) {
                color: var(--text-primary);
            }
        }

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: var(--space-4);
            margin-bottom: var(--space-6);
        }

        .stat-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-5);
            display: flex;
            flex-direction: column;
        }

        .stat-label {
            font-size: var(--text-sm);
            color: var(--text-muted);
            margin-bottom: var(--space-2);
        }

        .stat-value {
            font-size: var(--text-3xl);
            font-weight: var(--weight-bold);
            margin-bottom: var(--space-2);
        }

        .stat-change {
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);

            &.positive { color: var(--accent-green); }
            &.negative { color: var(--accent-red); }
        }

        .chart-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-6);
            margin-bottom: var(--space-6);

            h2 {
                font-size: var(--text-lg);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-6);
            }
        }

        .chart-box {
            height: 300px;
            display: flex;
            align-items: flex-end;
        }

        .bars {
            display: flex;
            justify-content: space-between;
            align-items: flex-end;
            width: 100%;
            height: 100%;
            gap: var(--space-2);
        }

        .bar-col {
            flex: 1;
            display: flex;
            flex-direction: column;
            align-items: center;
            height: 100%;
            justify-content: flex-end;
            gap: var(--space-2);
        }

        .bar {
            width: 100%;
            max-width: 40px;
            background: var(--gradient-brand);
            border-radius: var(--radius-sm) var(--radius-sm) 0 0;
            transition: height 1s var(--ease-out);
        }

        .bar-label {
            font-size: var(--text-xs);
            color: var(--text-muted);
        }

        .secondary-charts {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: var(--space-6);

            @media (max-width: 768px) {
                grid-template-columns: 1fr;
            }
        }

        .list-chart {
            display: flex;
            flex-direction: column;
            gap: var(--space-4);
        }

        .list-item {
            display: grid;
            grid-template-columns: 100px 1fr 60px;
            align-items: center;
            gap: var(--space-3);
        }

        .item-name {
            font-size: var(--text-sm);
            color: var(--text-secondary);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .item-bar-bg {
            height: var(--space-2);
            background: var(--bg-elevated);
            border-radius: var(--radius-full);
            overflow: hidden;
        }

        .item-bar {
            height: 100%;
            background: var(--accent-cyan);
            border-radius: var(--radius-full);

            &.error {
                background: var(--accent-red);
            }
        }

        .item-value {
            font-size: var(--text-sm);
            color: var(--text-muted);
            text-align: right;
            font-family: var(--font-mono);
        }
    `]
})
export class AnalyticsComponent {
    trafficData = signal([
        { label: '00:00', value: 20 },
        { label: '04:00', value: 15 },
        { label: '08:00', value: 45 },
        { label: '12:00', value: 85 },
        { label: '16:00', value: 65 },
        { label: '20:00', value: 50 },
    ]);

    topTools = signal([
        { name: 'read_file', count: 12500, percentage: 85 },
        { name: 'list_dir', count: 8200, percentage: 65 },
        { name: 'write_file', count: 5100, percentage: 40 },
        { name: 'search', count: 3200, percentage: 25 },
    ]);

    errorsByType = signal([
        { type: 'Rate Limit', count: 124, percentage: 70 },
        { type: 'Auth Failed', count: 45, percentage: 35 },
        { type: 'Timeout', count: 12, percentage: 10 },
        { type: 'Internal', count: 5, percentage: 5 },
    ]);
}
