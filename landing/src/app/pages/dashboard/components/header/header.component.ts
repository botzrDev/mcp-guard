import { Component, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { AuthService } from '../../../../core/auth';

@Component({
    selector: 'app-header',
    standalone: true,
    imports: [CommonModule, RouterModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <header class="dashboard-header">
            <div class="header-left">
                <button class="mobile-menu-btn" (click)="toggleMobileMenu()">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M4 6h16M4 12h16M4 18h16"/>
                    </svg>
                </button>
            </div>

            <div class="header-right">
                <div class="user-menu" (click)="toggleUserMenu()">
                    @if (authService.user()?.avatar_url) {
                        <img [src]="authService.user()?.avatar_url" alt="Avatar" class="user-avatar" />
                    } @else {
                        <div class="user-avatar-placeholder">
                            {{ authService.user()?.name?.charAt(0) || 'U' }}
                        </div>
                    }
                    <div class="user-info">
                        <span class="user-name">{{ authService.user()?.name }}</span>
                        <span class="user-role">{{ authService.user()?.role === 'admin' ? 'Admin' : 'User' }}</span>
                    </div>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="chevron">
                        <path d="M6 9l6 6 6-6"/>
                    </svg>
                </div>

                @if (isUserMenuOpen()) {
                    <div class="user-dropdown" (click)="$event.stopPropagation()">
                        <div class="dropdown-header">
                            <span class="dropdown-email">{{ authService.user()?.email }}</span>
                            <span class="dropdown-provider">via {{ authService.user()?.provider }}</span>
                        </div>
                        <div class="dropdown-divider"></div>
                        <a routerLink="/dashboard/license" class="dropdown-item" (click)="closeUserMenu()">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/>
                            </svg>
                            License
                        </a>
                        <a routerLink="/" class="dropdown-item" (click)="closeUserMenu()">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z"/>
                            </svg>
                            Home
                        </a>
                        <div class="dropdown-divider"></div>
                        <button class="dropdown-item danger" (click)="logout()">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4"/>
                                <polyline points="16 17 21 12 16 7"/>
                                <line x1="21" y1="12" x2="9" y2="12"/>
                            </svg>
                            Sign out
                        </button>
                    </div>
                }
            </div>
        </header>

        @if (isUserMenuOpen()) {
            <div class="backdrop" (click)="closeUserMenu()"></div>
        }
    `,
    styles: [`
        .dashboard-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: var(--space-4) var(--space-6);
            background: var(--bg-secondary);
            border-bottom: 1px solid var(--border-subtle);

            @media (max-width: 768px) {
                padding: var(--space-3) var(--space-4);
            }
        }

        .header-left {
            display: flex;
            align-items: center;
            gap: var(--space-4);
        }

        .mobile-menu-btn {
            display: none;
            width: var(--space-10);
            height: var(--space-10);
            align-items: center;
            justify-content: center;
            background: transparent;
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);
            color: var(--text-secondary);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-md);
                height: var(--icon-md);
            }

            &:hover {
                background: var(--bg-elevated);
                color: var(--text-primary);
            }

            @media (max-width: 768px) {
                display: flex;
            }
        }

        .header-right {
            position: relative;
        }

        .user-menu {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-2) var(--space-3);
            background: var(--bg-elevated);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            &:hover {
                border-color: var(--border-accent);
            }
        }

        .user-avatar {
            width: var(--space-9);
            height: var(--space-9);
            border-radius: var(--radius-full);
            object-fit: cover;
        }

        .user-avatar-placeholder {
            width: var(--space-9);
            height: var(--space-9);
            border-radius: var(--radius-full);
            background: var(--gradient-brand);
            color: var(--bg-primary);
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: var(--text-sm);
            font-weight: var(--weight-bold);
        }

        .user-info {
            display: flex;
            flex-direction: column;
            gap: var(--space-0-5);

            @media (max-width: 640px) {
                display: none;
            }
        }

        .user-name {
            font-size: var(--text-sm);
            font-weight: var(--weight-medium);
            color: var(--text-primary);
        }

        .user-role {
            font-size: var(--text-xs);
            color: var(--text-muted);
        }

        .chevron {
            width: var(--icon-sm);
            height: var(--icon-sm);
            color: var(--text-muted);

            @media (max-width: 640px) {
                display: none;
            }
        }

        .user-dropdown {
            position: absolute;
            top: calc(100% + var(--space-2));
            right: 0;
            width: 240px;
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            box-shadow: var(--shadow-xl);
            z-index: var(--z-dropdown);
            overflow: hidden;
        }

        .dropdown-header {
            padding: var(--space-4);
            display: flex;
            flex-direction: column;
            gap: var(--space-0-5);
        }

        .dropdown-email {
            font-size: var(--text-sm);
            font-weight: var(--weight-medium);
            color: var(--text-primary);
        }

        .dropdown-provider {
            font-size: var(--text-xs);
            color: var(--text-muted);
            text-transform: capitalize;
        }

        .dropdown-divider {
            height: 1px;
            background: var(--border-subtle);
        }

        .dropdown-item {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-3) var(--space-4);
            font-size: var(--text-sm);
            color: var(--text-secondary);
            text-decoration: none;
            background: transparent;
            border: none;
            width: 100%;
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

            &.danger {
                color: var(--accent-red);

                &:hover {
                    background: rgba(239, 68, 68, 0.1);
                }
            }
        }

        .backdrop {
            position: fixed;
            inset: 0;
            z-index: calc(var(--z-dropdown) - 1);
        }
    `]
})
export class HeaderComponent {
    authService = inject(AuthService);

    isUserMenuOpen = signal(false);
    isMobileMenuOpen = signal(false);

    toggleUserMenu(): void {
        this.isUserMenuOpen.update(v => !v);
    }

    closeUserMenu(): void {
        this.isUserMenuOpen.set(false);
    }

    toggleMobileMenu(): void {
        this.isMobileMenuOpen.update(v => !v);
    }

    logout(): void {
        this.closeUserMenu();
        this.authService.logout();
    }
}
