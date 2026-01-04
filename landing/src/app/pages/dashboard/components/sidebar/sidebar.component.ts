import { Component, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { AuthService } from '../../../../core/auth';

interface NavItem {
    label: string;
    icon: string;
    route: string;
    adminOnly?: boolean;
}

@Component({
    selector: 'app-sidebar',
    standalone: true,
    imports: [CommonModule, RouterModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <aside class="sidebar" [class.collapsed]="isCollapsed()">
            <div class="sidebar-header">
                <a routerLink="/" class="logo-link">
                    <img src="gateway.png" alt="MCP Guard" class="logo-img" />
                    <span class="logo-name" [class.hidden]="isCollapsed()">mcp-guard</span>
                </a>
                <button class="collapse-btn" (click)="toggleCollapse()" [attr.aria-label]="isCollapsed() ? 'Expand sidebar' : 'Collapse sidebar'">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        @if (isCollapsed()) {
                            <path d="M9 18l6-6-6-6"/>
                        } @else {
                            <path d="M15 18l-6-6 6-6"/>
                        }
                    </svg>
                </button>
            </div>

            <nav class="sidebar-nav">
                <div class="nav-section">
                    <span class="nav-section-label" [class.hidden]="isCollapsed()">Main</span>
                    @for (item of mainNavItems; track item.route) {
                        <a
                            [routerLink]="item.route"
                            routerLinkActive="active"
                            [routerLinkActiveOptions]="{ exact: item.route === '/dashboard/overview' }"
                            class="nav-item"
                            [title]="item.label"
                        >
                            <span class="nav-icon" [innerHTML]="getIcon(item.icon)"></span>
                            <span class="nav-label" [class.hidden]="isCollapsed()">{{ item.label }}</span>
                        </a>
                    }
                </div>

                @if (authService.isAdmin()) {
                    <div class="nav-section">
                        <span class="nav-section-label" [class.hidden]="isCollapsed()">Admin</span>
                        @for (item of adminNavItems; track item.route) {
                            <a
                                [routerLink]="item.route"
                                routerLinkActive="active"
                                class="nav-item admin"
                                [title]="item.label"
                            >
                                <span class="nav-icon" [innerHTML]="getIcon(item.icon)"></span>
                                <span class="nav-label" [class.hidden]="isCollapsed()">{{ item.label }}</span>
                            </a>
                        }
                    </div>
                }
            </nav>

            <div class="sidebar-footer">
                <a routerLink="/docs" class="nav-item" title="Documentation">
                    <span class="nav-icon" [innerHTML]="getIcon('docs')"></span>
                    <span class="nav-label" [class.hidden]="isCollapsed()">Docs</span>
                </a>
                <a href="https://github.com/botzrdev/mcp-guard" target="_blank" class="nav-item" title="GitHub">
                    <span class="nav-icon" [innerHTML]="getIcon('github')"></span>
                    <span class="nav-label" [class.hidden]="isCollapsed()">GitHub</span>
                </a>
            </div>
        </aside>
    `,
    styles: [`
        .sidebar {
            width: 260px;
            background: var(--bg-secondary);
            border-right: 1px solid var(--border-subtle);
            display: flex;
            flex-direction: column;
            transition: width var(--duration-normal) var(--ease-out);

            &.collapsed {
                width: 72px;
            }

            @media (max-width: 768px) {
                position: fixed;
                left: 0;
                top: 0;
                bottom: 0;
                z-index: var(--z-fixed);
                transform: translateX(-100%);

                &.collapsed {
                    width: 260px;
                }
            }
        }

        .sidebar-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: var(--space-4);
            border-bottom: 1px solid var(--border-subtle);
        }

        .logo-link {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            text-decoration: none;
            color: var(--text-primary);
        }

        .logo-img {
            width: var(--space-10);
            height: var(--space-10);
            object-fit: contain;
        }

        .logo-name {
            font-family: var(--font-mono);
            font-weight: var(--weight-bold);
            font-size: var(--text-base);
            white-space: nowrap;
            transition: opacity var(--duration-fast) var(--ease-out);

            &.hidden {
                opacity: 0;
                width: 0;
                overflow: hidden;
            }
        }

        .collapse-btn {
            width: var(--space-8);
            height: var(--space-8);
            display: flex;
            align-items: center;
            justify-content: center;
            background: transparent;
            border: none;
            border-radius: var(--radius-lg);
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

            @media (max-width: 768px) {
                display: none;
            }
        }

        .sidebar-nav {
            flex: 1;
            padding: var(--space-4);
            overflow-y: auto;
        }

        .nav-section {
            margin-bottom: var(--space-6);

            &:last-child {
                margin-bottom: 0;
            }
        }

        .nav-section-label {
            display: block;
            font-size: var(--text-xs);
            font-weight: var(--weight-semibold);
            text-transform: uppercase;
            letter-spacing: var(--tracking-wider);
            color: var(--text-muted);
            padding: var(--space-2) var(--space-3);
            transition: opacity var(--duration-fast) var(--ease-out);

            &.hidden {
                opacity: 0;
                height: 0;
                padding: 0;
                overflow: hidden;
            }
        }

        .nav-item {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-2-5) var(--space-3);
            text-decoration: none;
            color: var(--text-secondary);
            border-radius: var(--radius-lg);
            margin-bottom: var(--space-1);
            transition: all var(--duration-fast) var(--ease-out);

            &:hover {
                background: var(--bg-elevated);
                color: var(--text-primary);
            }

            &.active {
                background: rgba(78, 205, 196, 0.1);
                color: var(--accent-cyan);

                .nav-icon {
                    color: var(--accent-cyan);
                }
            }

            &.admin {
                &.active {
                    background: rgba(249, 115, 22, 0.1);
                    color: var(--accent-orange);

                    .nav-icon {
                        color: var(--accent-orange);
                    }
                }
            }
        }

        .nav-icon {
            width: var(--icon-md);
            height: var(--icon-md);
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;

            :deep(svg) {
                width: 100%;
                height: 100%;
            }
        }

        .nav-label {
            font-size: var(--text-sm);
            font-weight: var(--weight-medium);
            white-space: nowrap;
            transition: opacity var(--duration-fast) var(--ease-out);

            &.hidden {
                opacity: 0;
                width: 0;
                overflow: hidden;
            }
        }

        .sidebar-footer {
            padding: var(--space-4);
            border-top: 1px solid var(--border-subtle);
        }
    `]
})
export class SidebarComponent {
    authService = inject(AuthService);

    isCollapsed = signal(false);

    mainNavItems: NavItem[] = [
        { label: 'Overview', icon: 'home', route: '/dashboard/overview' },
        { label: 'License', icon: 'key', route: '/dashboard/license' },
        { label: 'API Keys', icon: 'lock', route: '/dashboard/api-keys' },
        { label: 'Usage', icon: 'chart', route: '/dashboard/usage' },
        { label: 'Quick Start', icon: 'rocket', route: '/dashboard/quickstart' },
    ];

    adminNavItems: NavItem[] = [
        { label: 'Users', icon: 'users', route: '/dashboard/admin/users' },
        { label: 'Analytics', icon: 'analytics', route: '/dashboard/admin/analytics' },
        { label: 'Audit Log', icon: 'audit', route: '/dashboard/admin/audit' },
        { label: 'Health', icon: 'health', route: '/dashboard/admin/health' },
    ];

    toggleCollapse(): void {
        this.isCollapsed.update(v => !v);
    }

    getIcon(name: string): string {
        const icons: Record<string, string> = {
            home: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>',
            key: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/></svg>',
            lock: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0110 0v4"/></svg>',
            chart: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>',
            rocket: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 00-2.91-.09z"/><path d="M12 15l-3-3a22 22 0 012-3.95A12.88 12.88 0 0122 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 01-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>',
            users: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 00-3-3.87"/><path d="M16 3.13a4 4 0 010 7.75"/></svg>',
            analytics: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 21H4.6c-.6 0-.9 0-1.1-.1a1 1 0 01-.4-.4c-.1-.2-.1-.5-.1-1.1V3"/><path d="M7 14l4-4 4 4 6-6"/></svg>',
            audit: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>',
            health: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>',
            docs: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>',
            github: '<svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>',
        };
        return icons[name] || '';
    }
}
