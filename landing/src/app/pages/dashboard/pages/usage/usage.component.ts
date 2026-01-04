import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
    selector: 'app-usage',
    standalone: true,
    imports: [CommonModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="usage-page">
            <div class="page-header">
                <h1>Usage</h1>
                <p class="page-subtitle">Monitor your API usage and performance metrics.</p>
            </div>

            <div class="period-selector">
                <button
                    [class.active]="selectedPeriod() === 'current_month'"
                    (click)="selectedPeriod.set('current_month')"
                >
                    This Month
                </button>
                <button
                    [class.active]="selectedPeriod() === 'last_30_days'"
                    (click)="selectedPeriod.set('last_30_days')"
                >
                    Last 30 Days
                </button>
            </div>

            <div class="usage-overview">
                <div class="usage-card primary">
                    <div class="usage-header">
                        <span class="usage-label">API Calls</span>
                        <span class="usage-period">{{ selectedPeriod() === 'current_month' ? 'This month' : 'Last 30 days' }}</span>
                    </div>
                    <div class="usage-value">{{ apiCalls() | number }}</div>
                    <div class="usage-limit">
                        <div class="limit-bar">
                            <div class="limit-fill" [style.width.%]="usagePercentage()"></div>
                        </div>
                        <span class="limit-text">{{ apiCalls() | number }} / {{ apiCallsLimit() | number }}</span>
                    </div>
                </div>

                <div class="stats-grid">
                    <div class="stat-card">
                        <span class="stat-label">Success Rate</span>
                        <span class="stat-value success">{{ successRate() }}%</span>
                    </div>
                    <div class="stat-card">
                        <span class="stat-label">Avg Latency</span>
                        <span class="stat-value">{{ avgLatency() }}ms</span>
                    </div>
                    <div class="stat-card">
                        <span class="stat-label">Unique Tools</span>
                        <span class="stat-value">{{ uniqueTools() }}</span>
                    </div>
                    <div class="stat-card">
                        <span class="stat-label">Rate Limited</span>
                        <span class="stat-value warning">{{ rateLimited() }}</span>
                    </div>
                </div>
            </div>

            <div class="tools-section">
                <h2>Tool Usage</h2>
                <div class="tools-table">
                    <div class="table-header">
                        <span class="col-tool">Tool</span>
                        <span class="col-calls">Calls</span>
                        <span class="col-success">Success Rate</span>
                        <span class="col-latency">Avg Latency</span>
                    </div>
                    @for (tool of toolUsage(); track tool.name) {
                        <div class="table-row">
                            <span class="col-tool">
                                <code>{{ tool.name }}</code>
                            </span>
                            <span class="col-calls">{{ tool.calls | number }}</span>
                            <span class="col-success" [class.success]="tool.successRate >= 99" [class.warning]="tool.successRate < 99 && tool.successRate >= 95" [class.error]="tool.successRate < 95">
                                {{ tool.successRate }}%
                            </span>
                            <span class="col-latency">{{ tool.avgLatency }}ms</span>
                        </div>
                    }
                </div>
            </div>

            <div class="history-section">
                <h2>Daily Usage</h2>
                <div class="chart-placeholder">
                    <div class="bars">
                        @for (day of dailyUsage(); track day.date) {
                            <div class="bar-container" [title]="day.date + ': ' + day.calls + ' calls'">
                                <div class="bar" [style.height.%]="(day.calls / maxDailyCalls()) * 100"></div>
                                <span class="bar-label">{{ day.dayName }}</span>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
    `,
    styles: [`
        .usage-page {
            max-width: 1000px;
            margin: 0 auto;
        }

        .page-header {
            margin-bottom: var(--space-6);

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

        .period-selector {
            display: flex;
            gap: var(--space-2);
            margin-bottom: var(--space-6);

            button {
                padding: var(--space-2) var(--space-4);
                background: var(--bg-secondary);
                border: 1px solid var(--border-subtle);
                border-radius: var(--radius-lg);
                font-size: var(--text-sm);
                font-weight: var(--weight-medium);
                color: var(--text-secondary);
                cursor: pointer;
                transition: all var(--duration-fast) var(--ease-out);

                &:hover {
                    background: var(--bg-elevated);
                    color: var(--text-primary);
                }

                &.active {
                    background: var(--text-primary);
                    color: var(--bg-primary);
                    border-color: transparent;
                }
            }
        }

        .usage-overview {
            margin-bottom: var(--space-8);
        }

        .usage-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-6);
            margin-bottom: var(--space-4);

            &.primary {
                background: linear-gradient(135deg, rgba(78, 205, 196, 0.05) 0%, rgba(59, 130, 246, 0.05) 100%);
                border-color: rgba(78, 205, 196, 0.2);
            }
        }

        .usage-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: var(--space-2);
        }

        .usage-label {
            font-size: var(--text-sm);
            font-weight: var(--weight-medium);
            color: var(--text-muted);
        }

        .usage-period {
            font-size: var(--text-xs);
            color: var(--text-dim);
        }

        .usage-value {
            font-size: var(--text-4xl);
            font-weight: var(--weight-bold);
            margin-bottom: var(--space-4);
        }

        .usage-limit {
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
        }

        .limit-bar {
            height: var(--space-2);
            background: var(--bg-elevated);
            border-radius: var(--radius-full);
            overflow: hidden;
        }

        .limit-fill {
            height: 100%;
            background: var(--gradient-brand);
            border-radius: var(--radius-full);
            transition: width var(--duration-normal) var(--ease-out);
        }

        .limit-text {
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
            gap: var(--space-4);
        }

        .stat-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-5);
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
        }

        .stat-label {
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .stat-value {
            font-size: var(--text-2xl);
            font-weight: var(--weight-bold);

            &.success {
                color: var(--accent-green);
            }

            &.warning {
                color: var(--accent-orange);
            }

            &.error {
                color: var(--accent-red);
            }
        }

        .tools-section,
        .history-section {
            margin-bottom: var(--space-8);

            h2 {
                font-size: var(--text-xl);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-4);
            }
        }

        .tools-table {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            overflow: hidden;
        }

        .table-header,
        .table-row {
            display: grid;
            grid-template-columns: 2fr 1fr 1fr 1fr;
            align-items: center;
            padding: var(--space-4);
            gap: var(--space-4);

            @media (max-width: 640px) {
                grid-template-columns: 1fr 1fr;
            }
        }

        .table-header {
            background: var(--bg-elevated);
            border-bottom: 1px solid var(--border-subtle);
            font-size: var(--text-xs);
            font-weight: var(--weight-semibold);
            text-transform: uppercase;
            letter-spacing: var(--tracking-wider);
            color: var(--text-muted);
        }

        .table-row {
            border-bottom: 1px solid var(--border-subtle);

            &:last-child {
                border-bottom: none;
            }
        }

        .col-tool code {
            font-family: var(--font-mono);
            font-size: var(--text-sm);
            color: var(--accent-cyan);
        }

        .col-calls,
        .col-success,
        .col-latency {
            font-size: var(--text-sm);
        }

        .col-success {
            font-weight: var(--weight-medium);

            &.success { color: var(--accent-green); }
            &.warning { color: var(--accent-orange); }
            &.error { color: var(--accent-red); }
        }

        .chart-placeholder {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            padding: var(--space-6);
            height: 200px;
        }

        .bars {
            display: flex;
            align-items: flex-end;
            justify-content: space-between;
            height: 100%;
            gap: var(--space-2);
        }

        .bar-container {
            flex: 1;
            display: flex;
            flex-direction: column;
            align-items: center;
            height: 100%;
            gap: var(--space-2);
        }

        .bar {
            width: 100%;
            max-width: 40px;
            background: var(--gradient-brand);
            border-radius: var(--radius-sm) var(--radius-sm) 0 0;
            min-height: 4px;
            transition: height var(--duration-normal) var(--ease-out);
        }

        .bar-label {
            font-size: var(--text-xs);
            color: var(--text-muted);
        }
    `]
})
export class UsageComponent {
    selectedPeriod = signal<'current_month' | 'last_30_days'>('current_month');

    apiCalls = signal(12345);
    apiCallsLimit = signal(50000);
    successRate = signal(99.8);
    avgLatency = signal(45);
    uniqueTools = signal(8);
    rateLimited = signal(23);

    usagePercentage(): number {
        return (this.apiCalls() / this.apiCallsLimit()) * 100;
    }

    toolUsage = signal([
        { name: 'read_file', calls: 5234, successRate: 99.9, avgLatency: 32 },
        { name: 'write_file', calls: 3421, successRate: 99.7, avgLatency: 45 },
        { name: 'list_directory', calls: 2156, successRate: 100, avgLatency: 28 },
        { name: 'execute_command', calls: 987, successRate: 98.5, avgLatency: 120 },
        { name: 'search_files', calls: 547, successRate: 99.2, avgLatency: 85 },
    ]);

    dailyUsage = signal([
        { date: '2026-01-01', dayName: 'Mon', calls: 1234 },
        { date: '2026-01-02', dayName: 'Tue', calls: 1567 },
        { date: '2026-01-03', dayName: 'Wed', calls: 1890 },
        { date: '2026-01-04', dayName: 'Thu', calls: 1456 },
        { date: '2026-01-05', dayName: 'Fri', calls: 1678 },
        { date: '2026-01-06', dayName: 'Sat', calls: 890 },
        { date: '2026-01-07', dayName: 'Sun', calls: 567 },
    ]);

    maxDailyCalls(): number {
        return Math.max(...this.dailyUsage().map(d => d.calls));
    }
}
