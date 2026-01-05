import { Injectable, signal, computed, inject } from '@angular/core';
import { Router } from '@angular/router';
import { User, JwtPayload, UserRole } from './auth.models';
import { environment } from '../../../environments/environment';

const TOKEN_KEY = 'mcp_guard_token';
const API_BASE = environment.apiUrl;

@Injectable({ providedIn: 'root' })
export class AuthService {
    private router = inject(Router);

    private currentUser = signal<User | null>(null);
    private loading = signal(false);
    private error = signal<string | null>(null);

    readonly user = this.currentUser.asReadonly();
    readonly isLoading = this.loading.asReadonly();
    readonly authError = this.error.asReadonly();
    readonly isAuthenticated = computed(() => !!this.currentUser());
    readonly isAdmin = computed(() => this.currentUser()?.role === 'admin');

    constructor() {
        this.loadStoredSession();
    }

    private loadStoredSession(): void {
        const token = this.getToken();
        if (token) {
            try {
                const payload = this.decodeToken(token);
                if (payload && !this.isTokenExpired(payload)) {
                    this.currentUser.set(this.payloadToUser(payload));
                } else {
                    this.clearSession();
                }
            } catch {
                this.clearSession();
            }
        }
    }

    private decodeToken(token: string): JwtPayload | null {
        try {
            const parts = token.split('.');
            if (parts.length !== 3) return null;

            const payload = JSON.parse(atob(parts[1]));
            return payload as JwtPayload;
        } catch {
            return null;
        }
    }

    private isTokenExpired(payload: JwtPayload): boolean {
        const now = Math.floor(Date.now() / 1000);
        return payload.exp < now;
    }

    private payloadToUser(payload: JwtPayload): User {
        return {
            id: payload.sub,
            email: payload.email,
            name: payload.name,
            avatar_url: payload.avatar_url,
            role: payload.role || 'user',
            provider: payload.provider,
            created_at: new Date(payload.iat * 1000).toISOString(),
        };
    }

    loginWithGitHub(): void {
        this.loading.set(true);
        this.error.set(null);
        const returnUrl = encodeURIComponent(window.location.origin + '/auth/callback');
        window.location.href = `${API_BASE}/oauth/authorize?provider=github&redirect_uri=${returnUrl}`;
    }

    loginWithGoogle(): void {
        this.loading.set(true);
        this.error.set(null);
        const returnUrl = encodeURIComponent(window.location.origin + '/auth/callback');
        window.location.href = `${API_BASE}/oauth/authorize?provider=google&redirect_uri=${returnUrl}`;
    }

    private magicLinkSentState = signal(false);
    private magicLinkEmailState = signal<string | null>(null);
    readonly magicLinkSent = this.magicLinkSentState.asReadonly();
    readonly magicLinkEmail = this.magicLinkEmailState.asReadonly();

    async sendMagicLink(email: string): Promise<boolean> {
        this.loading.set(true);
        this.error.set(null);
        this.magicLinkSentState.set(false);

        try {
            const response = await fetch(`${API_BASE}/auth/magic-link`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    email,
                    redirect_uri: window.location.origin + '/auth/callback'
                }),
            });

            if (!response.ok) {
                const data = await response.json().catch(() => ({}));
                this.error.set(data.message || 'Failed to send magic link');
                this.loading.set(false);
                return false;
            }

            this.magicLinkSentState.set(true);
            this.magicLinkEmailState.set(email);
            this.loading.set(false);
            return true;
        } catch {
            this.error.set('Network error. Please try again.');
            this.loading.set(false);
            return false;
        }
    }

    resetMagicLinkState(): void {
        this.magicLinkSentState.set(false);
        this.magicLinkEmailState.set(null);
        this.error.set(null);
    }

    handleCallback(token: string): boolean {
        try {
            const payload = this.decodeToken(token);
            if (!payload) {
                this.error.set('Invalid token received');
                return false;
            }

            if (this.isTokenExpired(payload)) {
                this.error.set('Token has expired');
                return false;
            }

            localStorage.setItem(TOKEN_KEY, token);
            this.currentUser.set(this.payloadToUser(payload));
            this.loading.set(false);
            return true;
        } catch {
            this.error.set('Failed to process authentication');
            this.loading.set(false);
            return false;
        }
    }

    handleCallbackError(error: string): void {
        this.error.set(error);
        this.loading.set(false);
    }

    logout(): void {
        this.clearSession();
        this.router.navigate(['/']);
    }

    private clearSession(): void {
        localStorage.removeItem(TOKEN_KEY);
        this.currentUser.set(null);
    }

    getToken(): string | null {
        return localStorage.getItem(TOKEN_KEY);
    }

    hasRole(role: UserRole): boolean {
        const user = this.currentUser();
        if (!user) return false;
        if (role === 'user') return true;
        return user.role === role;
    }

    navigateToDashboard(): void {
        this.router.navigate(['/dashboard']);
    }
}
