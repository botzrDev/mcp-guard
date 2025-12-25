import { Routes } from '@angular/router';

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
                path: 'quickstart',
                loadComponent: () => import('./pages/docs/quickstart/quickstart.component').then(m => m.QuickstartComponent)
            },
            {
                path: 'configuration',
                loadComponent: () => import('./pages/docs/configuration/configuration.component').then(m => m.ConfigurationComponent)
            },
            {
                path: 'api',
                loadComponent: () => import('./pages/docs/api/api.component').then(m => m.ApiComponent)
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
    {
        path: '**',
        redirectTo: ''
    }
];
