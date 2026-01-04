import { Routes } from '@angular/router';
import { AuthGuard, RoleGuard, GuestGuard } from './core/auth';

export const routes: Routes = [
    {
        path: '',
        loadComponent: () => import('./pages/home/home.component').then(m => m.HomeComponent)
    },
    {
        path: 'docs',
        loadComponent: () => import('./pages/docs/docs.component').then(m => m.DocsComponent),
        children: [
            {
                path: '',
                redirectTo: 'quickstart',
                pathMatch: 'full'
            },
            {
                path: ':slug',
                loadComponent: () => import('./pages/docs/doc-page/doc-page.component').then(m => m.DocPageComponent)
            }
        ]
    },
    {
        path: 'changelog',
        loadComponent: () => import('./pages/changelog/changelog.component').then(m => m.ChangelogComponent)
    },
    {
        path: 'blog',
        loadComponent: () => import('./pages/blog/blog.component').then(m => m.BlogComponent)
    },
    {
        path: 'about',
        loadComponent: () => import('./pages/about/about.component').then(m => m.AboutComponent)
    },
    {
        path: 'contact',
        loadComponent: () => import('./pages/contact/contact.component').then(m => m.ContactComponent)
    },
    {
        path: 'privacy',
        loadComponent: () => import('./pages/legal/privacy.component').then(m => m.PrivacyComponent)
    },
    {
        path: 'terms',
        loadComponent: () => import('./pages/legal/terms.component').then(m => m.TermsComponent)
    },
    // Auth routes (public, guest only)
    {
        path: 'login',
        loadComponent: () => import('./pages/auth/login/login.component').then(m => m.LoginComponent),
        canActivate: [GuestGuard]
    },
    {
        path: 'auth/callback',
        loadComponent: () => import('./pages/auth/callback/callback.component').then(m => m.CallbackComponent)
    },
    // Signup and success routes (public)
    {
        path: 'signup',
        loadComponent: () => import('./pages/signup/signup.component').then(m => m.SignupComponent)
    },
    {
        path: 'success',
        loadComponent: () => import('./pages/success/success.component').then(m => m.SuccessComponent)
    },
    // Dashboard routes (protected)
    {
        path: 'dashboard',
        loadComponent: () => import('./pages/dashboard/dashboard.component').then(m => m.DashboardComponent),
        canActivate: [AuthGuard],
        children: [
            {
                path: '',
                redirectTo: 'overview',
                pathMatch: 'full'
            },
            {
                path: 'overview',
                loadComponent: () => import('./pages/dashboard/pages/overview/overview.component').then(m => m.OverviewComponent)
            },
            {
                path: 'license',
                loadComponent: () => import('./pages/dashboard/pages/license/license.component').then(m => m.LicenseComponent)
            },
            {
                path: 'api-keys',
                loadComponent: () => import('./pages/dashboard/pages/api-keys/api-keys.component').then(m => m.ApiKeysComponent)
            },
            {
                path: 'usage',
                loadComponent: () => import('./pages/dashboard/pages/usage/usage.component').then(m => m.UsageComponent)
            },
            {
                path: 'quickstart',
                loadComponent: () => import('./pages/dashboard/pages/quickstart/quickstart.component').then(m => m.QuickstartComponent)
            },
            // Admin routes (require admin role)
            {
                path: 'admin',
                canActivate: [RoleGuard],
                data: { roles: ['admin'] },
                children: [
                    {
                        path: '',
                        redirectTo: 'users',
                        pathMatch: 'full'
                    },
                    {
                        path: 'users',
                        loadComponent: () => import('./pages/dashboard/pages/admin/users/users.component').then(m => m.UsersComponent)
                    },
                    {
                        path: 'analytics',
                        loadComponent: () => import('./pages/dashboard/pages/admin/analytics/analytics.component').then(m => m.AnalyticsComponent)
                    },
                    {
                        path: 'audit',
                        loadComponent: () => import('./pages/dashboard/pages/admin/audit/audit.component').then(m => m.AuditComponent)
                    },
                    {
                        path: 'health',
                        loadComponent: () => import('./pages/dashboard/pages/admin/health/health.component').then(m => m.HealthComponent)
                    }
                ]
            }
        ]
    },
    {
        path: '**',
        redirectTo: ''
    }
];
