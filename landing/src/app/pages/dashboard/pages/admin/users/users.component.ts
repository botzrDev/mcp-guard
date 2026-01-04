import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

interface User {
    id: string;
    name: string;
    email: string;
    role: 'admin' | 'user';
    provider: 'github' | 'google';
    created_at: string;
    last_login: string;
    status: 'active' | 'suspended';
}

@Component({
    selector: 'app-users',
    standalone: true,
    imports: [CommonModule, FormsModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="users-page">
            <div class="page-header">
                <div class="header-content">
                    <h1>Users</h1>
                    <p class="page-subtitle">Manage system users and their roles.</p>
                </div>
                <div class="header-actions">
                    <div class="search-box">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="11" cy="11" r="8"/>
                            <path d="M21 21l-4.35-4.35"/>
                        </svg>
                        <input 
                            type="text" 
                            placeholder="Search users..." 
                            [(ngModel)]="searchQuery"
                            (input)="filterUsers()"
                        >
                    </div>
                </div>
            </div>

            <div class="users-table">
                <div class="table-header">
                    <span class="col-user">User</span>
                    <span class="col-role">Role</span>
                    <span class="col-provider">Provider</span>
                    <span class="col-joined">Joined</span>
                    <span class="col-status">Status</span>
                    <span class="col-actions">Actions</span>
                </div>
                @for (user of filteredUsers(); track user.id) {
                    <div class="table-row">
                        <div class="col-user">
                            <div class="user-avatar">
                                {{ user.name.charAt(0) }}
                            </div>
                            <div class="user-info">
                                <span class="user-name">{{ user.name }}</span>
                                <span class="user-email">{{ user.email }}</span>
                            </div>
                        </div>
                        <div class="col-role">
                            <span class="role-badge" [class]="user.role">
                                {{ user.role }}
                            </span>
                        </div>
                        <div class="col-provider">
                            <span class="provider-badge">
                                {{ user.provider }}
                            </span>
                        </div>
                        <div class="col-joined">
                            {{ formatDate(user.created_at) }}
                        </div>
                        <div class="col-status">
                            <span class="status-badge" [class]="user.status">
                                {{ user.status }}
                            </span>
                        </div>
                        <div class="col-actions">
                            <button class="action-btn" title="Edit User">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                                    <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                }
            </div>
        </div>
    `,
    styles: [`
        .users-page {
            max-width: 1200px;
            margin: 0 auto;
        }

        .page-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: var(--space-6);

            @media (max-width: 768px) {
                flex-direction: column;
                gap: var(--space-4);
                align-items: stretch;
            }

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

        .search-box {
            position: relative;
            width: 300px;

            @media (max-width: 768px) {
                width: 100%;
            }

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

        .users-table {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            overflow: hidden;
        }

        .table-header,
        .table-row {
            display: grid;
            grid-template-columns: 2fr 1fr 1fr 1fr 1fr auto;
            align-items: center;
            padding: var(--space-4);
            gap: var(--space-4);

            @media (max-width: 900px) {
                grid-template-columns: 2fr 1fr 1fr auto;
                
                .col-added, .col-provider {
                    display: none;
                }
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

        .col-user {
            display: flex;
            align-items: center;
            gap: var(--space-3);
        }

        .user-avatar {
            width: var(--space-10);
            height: var(--space-10);
            border-radius: var(--radius-full);
            background: var(--gradient-brand);
            color: var(--bg-primary);
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: var(--weight-bold);
            font-size: var(--text-lg);
        }

        .user-info {
            display: flex;
            flex-direction: column;
            gap: var(--space-0-5);
        }

        .user-name {
            font-weight: var(--weight-medium);
            font-size: var(--text-sm);
        }

        .user-email {
            font-size: var(--text-xs);
            color: var(--text-muted);
        }

        .role-badge {
            padding: var(--space-1) var(--space-2);
            border-radius: var(--radius-sm);
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);
            text-transform: capitalize;

            &.admin {
                background: rgba(249, 115, 22, 0.1);
                color: var(--accent-orange);
            }

            &.user {
                background: rgba(148, 163, 184, 0.1);
                color: var(--text-secondary);
            }
        }

        .provider-badge {
            font-size: var(--text-sm);
            text-transform: capitalize;
            color: var(--text-secondary);
        }

        .col-joined {
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .status-badge {
            padding: var(--space-1) var(--space-2);
            border-radius: var(--radius-full);
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);

            &.active {
                background: rgba(34, 197, 94, 0.1);
                color: var(--accent-green);
            }

            &.suspended {
                background: rgba(239, 68, 68, 0.1);
                color: var(--accent-red);
            }
        }

        .col-actions {
            display: flex;
            justify-content: flex-end;
        }

        .action-btn {
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
export class UsersComponent {
    searchQuery = '';

    users = signal<User[]>([
        {
            id: '1',
            name: 'Austin Green',
            email: 'austin@example.com',
            role: 'admin',
            provider: 'github',
            created_at: '2025-12-01T10:00:00Z',
            last_login: '2026-01-04T08:00:00Z',
            status: 'active'
        },
        {
            id: '2',
            name: 'Jane Doe',
            email: 'jane@example.com',
            role: 'user',
            provider: 'google',
            created_at: '2025-12-15T14:30:00Z',
            last_login: '2026-01-03T16:45:00Z',
            status: 'active'
        },
        {
            id: '3',
            name: 'John Smith',
            email: 'john@example.com',
            role: 'user',
            provider: 'github',
            created_at: '2026-01-02T09:15:00Z',
            last_login: '2026-01-02T09:15:00Z',
            status: 'suspended'
        }
    ]);

    filteredUsers = signal(this.users());

    filterUsers() {
        const query = this.searchQuery.toLowerCase();
        this.filteredUsers.set(
            this.users().filter(user =>
                user.name.toLowerCase().includes(query) ||
                user.email.toLowerCase().includes(query)
            )
        );
    }

    formatDate(dateStr: string): string {
        return new Date(dateStr).toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            year: 'numeric'
        });
    }
}
