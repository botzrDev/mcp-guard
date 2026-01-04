import { Component, OnInit, inject, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, Router, RouterLink } from '@angular/router';
import { AuthService } from '../../../core/auth';

@Component({
    selector: 'app-callback',
    standalone: true,
    imports: [CommonModule, RouterLink],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <section class="callback">
            <div class="callback-container">
                @if (error()) {
                    <div class="callback-card error">
                        <div class="icon-container error">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"/>
                                <line x1="15" y1="9" x2="9" y2="15"/>
                                <line x1="9" y1="9" x2="15" y2="15"/>
                            </svg>
                        </div>
                        <h1>Authentication Failed</h1>
                        <p class="message">{{ error() }}</p>
                        <a routerLink="/login" class="btn btn-primary">
                            Try Again
                        </a>
                    </div>
                } @else {
                    <div class="callback-card">
                        <div class="icon-container">
                            <div class="spinner"></div>
                        </div>
                        <h1>Signing you in...</h1>
                        <p class="message">Please wait while we complete the authentication.</p>
                    </div>
                }
            </div>
        </section>
    `,
    styles: [`
        .callback {
            min-height: 100vh;
            padding: var(--space-16) 0;
            background: var(--bg-primary);
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .callback-container {
            max-width: 480px;
            margin: 0 auto;
            padding: 0 var(--container-px);
        }

        .callback-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-3xl);
            padding: var(--space-12);
            text-align: center;

            &.error {
                border-color: rgba(239, 68, 68, 0.3);
            }
        }

        .icon-container {
            width: var(--space-20);
            height: var(--space-20);
            background: rgba(78, 205, 196, 0.1);
            border-radius: var(--radius-full);
            display: flex;
            align-items: center;
            justify-content: center;
            margin: 0 auto var(--space-6);

            &.error {
                background: rgba(239, 68, 68, 0.1);

                svg {
                    color: var(--accent-red);
                }
            }

            svg {
                width: var(--space-10);
                height: var(--space-10);
                color: var(--accent-cyan);
            }
        }

        .spinner {
            width: var(--space-10);
            height: var(--space-10);
            border: 3px solid var(--border-subtle);
            border-top-color: var(--accent-cyan);
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }

        @keyframes spin {
            to { transform: rotate(360deg); }
        }

        h1 {
            font-family: var(--font-display);
            font-size: var(--text-2xl);
            font-weight: var(--weight-bold);
            margin-bottom: var(--space-2);
        }

        .message {
            color: var(--text-muted);
            font-size: var(--text-base);
            margin-bottom: var(--space-6);
        }

        .btn {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            padding: var(--space-3) var(--space-6);
            border-radius: var(--radius-xl);
            font-size: var(--text-base);
            font-weight: var(--weight-semibold);
            text-decoration: none;
            transition: all var(--duration-normal) var(--ease-out);
        }

        .btn-primary {
            background: var(--text-primary);
            color: var(--bg-primary);

            &:hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow-lg);
            }
        }
    `]
})
export class CallbackComponent implements OnInit {
    private route = inject(ActivatedRoute);
    private router = inject(Router);
    private authService = inject(AuthService);

    error = signal<string | null>(null);

    ngOnInit(): void {
        this.route.queryParams.subscribe(params => {
            const token = params['token'];
            const errorParam = params['error'];

            if (errorParam) {
                const errorMessages: Record<string, string> = {
                    'access_denied': 'Access was denied by the provider.',
                    'invalid_state': 'Security validation failed. Please try again.',
                    'provider_error': 'The authentication provider returned an error.',
                };
                this.error.set(errorMessages[errorParam] || 'Authentication failed. Please try again.');
                return;
            }

            if (!token) {
                this.error.set('No authentication token received.');
                return;
            }

            const success = this.authService.handleCallback(token);
            if (success) {
                const returnUrl = params['returnUrl'] || '/dashboard';
                this.router.navigate([returnUrl]);
            } else {
                this.error.set(this.authService.authError() || 'Failed to process authentication.');
            }
        });
    }
}
