import { Component, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { SidebarComponent } from './components/sidebar/sidebar.component';
import { HeaderComponent } from './components/header/header.component';

@Component({
    selector: 'app-dashboard',
    standalone: true,
    imports: [CommonModule, RouterModule, SidebarComponent, HeaderComponent],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="dashboard-layout">
            <app-sidebar />
            <div class="dashboard-main">
                <app-header />
                <main class="dashboard-content">
                    <router-outlet />
                </main>
            </div>
        </div>
    `,
    styles: [`
        .dashboard-layout {
            display: flex;
            min-height: 100vh;
            background: var(--bg-primary);
        }

        .dashboard-main {
            flex: 1;
            display: flex;
            flex-direction: column;
            min-width: 0;
        }

        .dashboard-content {
            flex: 1;
            padding: var(--space-6);
            overflow-y: auto;

            @media (max-width: 768px) {
                padding: var(--space-4);
            }
        }
    `]
})
export class DashboardComponent {}
