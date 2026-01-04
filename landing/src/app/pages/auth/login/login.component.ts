import { Component, inject, OnInit, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { AuthService } from '../../../core/auth';

@Component({
    selector: 'app-login',
    standalone: true,
    imports: [CommonModule, RouterLink],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <section class="login">
            <div class="login-container">
                <div class="login-card">
                    <a routerLink="/" class="back-link">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M19 12H5M12 19l-7-7 7-7"/>
                        </svg>
                        Back to home
                    </a>

                    <div class="logo-block">
                        <img src="gateway.png" alt="MCP Guard" class="logo-img" />
                        <span class="logo-name">mcp-guard</span>
                    </div>

                    <h1>Welcome back</h1>
                    <p class="subtitle">Sign in to access your dashboard</p>

                    @if (errorMessage()) {
                        <div class="error-banner">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"/>
                                <line x1="12" y1="8" x2="12" y2="12"/>
                                <line x1="12" y1="16" x2="12.01" y2="16"/>
                            </svg>
                            {{ errorMessage() }}
                        </div>
                    }

                    <div class="oauth-buttons">
                        <button
                            class="oauth-btn github"
                            (click)="loginWithGitHub()"
                            [disabled]="authService.isLoading()"
                        >
                            <svg viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                            Continue with GitHub
                        </button>

                        <button
                            class="oauth-btn google"
                            (click)="loginWithGoogle()"
                            [disabled]="authService.isLoading()"
                        >
                            <svg viewBox="0 0 24 24">
                                <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                                <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                                <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                                <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                            </svg>
                            Continue with Google
                        </button>
                    </div>

                    <p class="terms">
                        By continuing, you agree to our
                        <a routerLink="/terms">Terms of Service</a> and
                        <a routerLink="/privacy">Privacy Policy</a>
                    </p>
                </div>

                <div class="features-list">
                    <h3>What you get with your account</h3>
                    <ul>
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                            </svg>
                            <div>
                                <strong>License management</strong>
                                <p>View and manage your Pro or Enterprise license</p>
                            </div>
                        </li>
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                                <path d="M7 11V7a5 5 0 0110 0v4"/>
                            </svg>
                            <div>
                                <strong>API key management</strong>
                                <p>Create and revoke API keys for your MCP servers</p>
                            </div>
                        </li>
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
                                <polyline points="17 8 12 3 7 8"/>
                                <line x1="12" y1="3" x2="12" y2="15"/>
                            </svg>
                            <div>
                                <strong>Usage analytics</strong>
                                <p>Track API calls and monitor your usage</p>
                            </div>
                        </li>
                    </ul>
                </div>
            </div>
        </section>
    `,
    styles: [`
        .login {
            min-height: 100vh;
            padding: var(--space-16) 0;
            background: var(--bg-primary);
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .login-container {
            max-width: 1000px;
            margin: 0 auto;
            padding: 0 var(--container-px);
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: var(--space-16);

            @media (max-width: 900px) {
                grid-template-columns: 1fr;
            }
        }

        .login-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-3xl);
            padding: var(--space-10);
        }

        .back-link {
            display: inline-flex;
            align-items: center;
            gap: var(--space-2);
            color: var(--text-muted);
            text-decoration: none;
            font-size: var(--text-sm);
            margin-bottom: var(--space-8);
            transition: color var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }

            &:hover {
                color: var(--text-primary);
            }
        }

        .logo-block {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            margin-bottom: var(--space-6);
        }

        .logo-img {
            width: var(--space-12);
            height: var(--space-12);
            object-fit: contain;
        }

        .logo-name {
            font-family: var(--font-mono);
            font-weight: var(--weight-bold);
            font-size: var(--text-xl);
        }

        h1 {
            font-family: var(--font-display);
            font-size: var(--text-3xl);
            font-weight: var(--weight-bold);
            margin-bottom: var(--space-2);
        }

        .subtitle {
            color: var(--text-muted);
            font-size: var(--text-base);
            margin-bottom: var(--space-8);
        }

        .error-banner {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-4);
            background: rgba(239, 68, 68, 0.1);
            border: 1px solid rgba(239, 68, 68, 0.2);
            border-radius: var(--radius-lg);
            color: var(--accent-red);
            margin-bottom: var(--space-6);
            font-size: var(--text-sm);

            svg {
                width: var(--icon-md);
                height: var(--icon-md);
                flex-shrink: 0;
            }
        }

        .oauth-buttons {
            display: flex;
            flex-direction: column;
            gap: var(--space-3);
            margin-bottom: var(--space-6);
        }

        .oauth-btn {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: var(--space-3);
            width: 100%;
            padding: var(--space-4) var(--space-6);
            border-radius: var(--radius-xl);
            font-size: var(--text-base);
            font-weight: var(--weight-semibold);
            cursor: pointer;
            transition: all var(--duration-normal) var(--ease-out);
            border: 1px solid var(--border-subtle);

            svg {
                width: var(--icon-lg);
                height: var(--icon-lg);
            }

            &:disabled {
                opacity: 0.6;
                cursor: not-allowed;
            }

            &:not(:disabled):hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow-lg);
            }
        }

        .oauth-btn.github {
            background: var(--text-primary);
            color: var(--bg-primary);
            border-color: transparent;
        }

        .oauth-btn.google {
            background: var(--bg-elevated);
            color: var(--text-primary);

            &:not(:disabled):hover {
                background: var(--bg-hover);
                border-color: var(--border-accent);
            }
        }

        .terms {
            text-align: center;
            font-size: var(--text-xs);
            color: var(--text-muted);

            a {
                color: var(--accent-cyan);
                text-decoration: none;

                &:hover {
                    text-decoration: underline;
                }
            }
        }

        .features-list {
            padding-top: var(--space-8);

            h3 {
                font-size: var(--text-xl);
                font-weight: var(--weight-semibold);
                margin-bottom: var(--space-6);
            }

            ul {
                list-style: none;
                display: flex;
                flex-direction: column;
                gap: var(--space-5);
            }

            li {
                display: flex;
                gap: var(--space-4);

                svg {
                    width: var(--icon-lg);
                    height: var(--icon-lg);
                    color: var(--accent-cyan);
                    flex-shrink: 0;
                    margin-top: var(--space-1);
                }

                strong {
                    display: block;
                    margin-bottom: var(--space-1);
                }

                p {
                    color: var(--text-muted);
                    font-size: var(--text-sm);
                }
            }
        }
    `]
})
export class LoginComponent implements OnInit {
    authService = inject(AuthService);
    private route = inject(ActivatedRoute);

    errorMessage = signal<string | null>(null);

    ngOnInit(): void {
        this.route.queryParams.subscribe(params => {
            if (params['error']) {
                const errorMessages: Record<string, string> = {
                    'session_expired': 'Your session has expired. Please sign in again.',
                    'access_denied': 'Access was denied. Please try again.',
                    'invalid_state': 'Authentication failed. Please try again.',
                };
                this.errorMessage.set(errorMessages[params['error']] || 'An error occurred. Please try again.');
            }
        });
    }

    loginWithGitHub(): void {
        this.authService.loginWithGitHub();
    }

    loginWithGoogle(): void {
        this.authService.loginWithGoogle();
    }
}
