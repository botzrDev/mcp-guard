import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

interface AuditLog {
    id: string;
    timestamp: string;
    actor: string;
    action: string;
    resource: string;
    status: 'success' | 'failure';
    ip_address: string;
    details?: string;
}

@Component({
    selector: 'app-audit',
    standalone: true,
    imports: [CommonModule, FormsModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="audit-page">
            <div class="page-header">
                <h1>Audit Log</h1>
                <div class="header-actions">
                    <button class="export-btn">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
                            <polyline points="7 10 12 15 17 10"/>
                            <line x1="12" y1="15" x2="12" y2="3"/>
                        </svg>
                        Export CSV
                    </button>
                    <div class="search-box">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="11" cy="11" r="8"/>
                            <path d="M21 21l-4.35-4.35"/>
                        </svg>
                        <input 
                            type="text" 
                            placeholder="Search logs..." 
                            [(ngModel)]="searchQuery"
                            (input)="filterLogs()"
                        >
                    </div>
                </div>
            </div>

            <div class="audit-table">
                <div class="table-header">
                    <span class="col-time">Timestamp</span>
                    <span class="col-actor">Actor</span>
                    <span class="col-action">Action</span>
                    <span class="col-resource">Resource</span>
                    <span class="col-status">Status</span>
                    <span class="col-ip">IP Address</span>
                    <span class="col-details"></span>
                </div>
                @for (log of filteredLogs(); track log.id) {
                    <div class="table-row">
                        <div class="col-time">
                            {{ formatTime(log.timestamp) }}
                        </div>
                        <div class="col-actor">
                            <div class="actor-badge">
                                {{ log.actor.charAt(0) }}
                            </div>
                            <span class="actor-name">{{ log.actor }}</span>
                        </div>
                        <div class="col-action">
                            <code>{{ log.action }}</code>
                        </div>
                        <div class="col-resource">
                            {{ log.resource }}
                        </div>
                        <div class="col-status">
                            <span class="status-dot" [class]="log.status"></span>
                            {{ log.status }}
                        </div>
                        <div class="col-ip">
                            {{ log.ip_address }}
                        </div>
                        <div class="col-details">
                            <button class="details-btn">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <circle cx="12" cy="12" r="1"/>
                                    <circle cx="12" cy="5" r="1"/>
                                    <circle cx="12" cy="19" r="1"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                }
            </div>
        </div>
    `,
    styles: [`
        .audit-page {
            max-width: 1200px;
            margin: 0 auto;
        }

        .page-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: var(--space-6);

            h1 {
                font-family: var(--font-display);
                font-size: var(--text-3xl);
                font-weight: var(--weight-bold);
            }
        }

        .header-actions {
            display: flex;
            gap: var(--space-3);
        }

        .export-btn {
            display: flex;
            align-items: center;
            gap: var(--space-2);
            padding: var(--space-2-5) var(--space-4);
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);
            color: var(--text-secondary);
            font-size: var(--text-sm);
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

        .search-box {
            position: relative;
            width: 300px;

            svg {
                position: absolute;
                left: var(--space-3);
                top: 50%;
                transform: translateY(-50%);
                width: var(--icon-sm);
                height: var(--icon-sm);
                color: var(--text-muted);
            }

            input {
                width: 100%;
                padding: var(--space-2-5) var(--space-4);
                padding-left: var(--space-10);
                background: var(--bg-secondary);
                border: 1px solid var(--border-subtle);
                border-radius: var(--radius-lg);
                font-size: var(--text-sm);
                color: var(--text-primary);
                transition: all var(--duration-fast) var(--ease-out);

                &:focus {
                    outline: none;
                    border-color: var(--accent-cyan);
                    background: var(--bg-elevated);
                }
            }
        }

        .audit-table {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            overflow: hidden;
        }

        .table-header,
        .table-row {
            display: grid;
            grid-template-columns: 180px 1.5fr 1fr 1fr 100px 140px 50px;
            align-items: center;
            padding: var(--space-3) var(--space-4);
            gap: var(--space-4);
            font-size: var(--text-sm);

            @media (max-width: 1024px) {
                grid-template-columns: 140px 1fr 1fr 100px auto;
                .col-resource, .col-ip { display: none; }
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
            transition: background var(--duration-fast) var(--ease-out);

            &:last-child {
                border-bottom: none;
            }

            &:hover {
                background: var(--bg-elevated);
            }
        }

        .col-time {
            color: var(--text-muted);
            font-variant-numeric: tabular-nums;
        }

        .col-actor {
            display: flex;
            align-items: center;
            gap: var(--space-2);
        }

        .actor-badge {
            width: var(--space-6);
            height: var(--space-6);
            background: var(--bg-elevated);
            color: var(--text-muted);
            border-radius: var(--radius-full);
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);
        }

        .actor-name {
            font-weight: var(--weight-medium);
        }

        .col-action code {
            font-family: var(--font-mono);
            font-size: var(--text-xs);
            color: var(--accent-cyan);
            background: rgba(78, 205, 196, 0.1);
            padding: var(--space-1) var(--space-2);
            border-radius: var(--radius-sm);
        }

        .col-resource {
            color: var(--text-secondary);
        }

        .col-status {
            display: flex;
            align-items: center;
            gap: var(--space-2);
            text-transform: capitalize;
        }

        .status-dot {
            width: 6px;
            height: 6px;
            border-radius: 50%;

            &.success { background: var(--accent-green); box-shadow: 0 0 6px var(--accent-green); }
            &.failure { background: var(--accent-red); box-shadow: 0 0 6px var(--accent-red); }
        }

        .col-ip {
            color: var(--text-muted);
            font-family: var(--font-mono);
            font-size: var(--text-xs);
        }

        .details-btn {
            background: transparent;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            padding: var(--space-1);
            border-radius: var(--radius-md);

            &:hover {
                background: var(--bg-active);
                color: var(--text-primary);
            }

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }
        }
    `]
})
export class AuditComponent {
    searchQuery = '';

    logs = signal<AuditLog[]>([
        {
            id: '1',
            timestamp: '2026-01-04T08:55:00Z',
            actor: 'admin',
            action: 'KEY_CREATE',
            resource: 'api_key_123',
            status: 'success',
            ip_address: '192.168.1.50'
        },
        {
            id: '2',
            timestamp: '2026-01-04T08:45:00Z',
            actor: 'system',
            action: 'RATE_LIMIT',
            resource: 'user_456',
            status: 'failure',
            ip_address: '10.0.0.5'
        },
        {
            id: '3',
            timestamp: '2026-01-04T08:30:00Z',
            actor: 'jane@example.com',
            action: 'LOGIN',
            resource: 'dashboard',
            status: 'success',
            ip_address: '172.16.0.1'
        },
        // More mock data...
    ]);

    filteredLogs = signal(this.logs());

    filterLogs() {
        const query = this.searchQuery.toLowerCase();
        this.filteredLogs.set(
            this.logs().filter(log =>
                log.actor.toLowerCase().includes(query) ||
                log.action.toLowerCase().includes(query) ||
                log.resource.toLowerCase().includes(query)
            )
        );
    }

    formatTime(timestamp: string): string {
        return new Date(timestamp).toLocaleString();
    }
}
