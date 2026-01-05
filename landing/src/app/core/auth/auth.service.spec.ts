import { TestBed } from '@angular/core/testing';
import { Router } from '@angular/router';
import { AuthService } from './auth.service';

describe('AuthService', () => {
    let service: AuthService;
    let routerSpy: jest.Mocked<Router>;
    let localStorageMock: { [key: string]: string };

    // Helper to create a valid JWT token
    const createToken = (payload: object, expired = false): string => {
        const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
        const exp = expired
            ? Math.floor(Date.now() / 1000) - 3600 // 1 hour ago
            : Math.floor(Date.now() / 1000) + 3600; // 1 hour from now
        const fullPayload = btoa(
            JSON.stringify({
                ...payload,
                exp,
                iat: Math.floor(Date.now() / 1000),
            })
        );
        const signature = btoa('fake-signature');
        return `${header}.${fullPayload}.${signature}`;
    };

    const mockUserPayload = {
        sub: 'user-123',
        email: 'test@example.com',
        name: 'Test User',
        avatar_url: 'https://example.com/avatar.jpg',
        role: 'user' as const,
        provider: 'github',
    };

    const mockAdminPayload = {
        ...mockUserPayload,
        sub: 'admin-123',
        email: 'admin@example.com',
        name: 'Admin User',
        role: 'admin' as const,
    };

    beforeEach(() => {
        // Reset localStorage mock
        localStorageMock = {};
        jest.spyOn(Storage.prototype, 'getItem').mockImplementation(
            (key) => localStorageMock[key] || null
        );
        jest.spyOn(Storage.prototype, 'setItem').mockImplementation(
            (key, value) => {
                localStorageMock[key] = value;
            }
        );
        jest.spyOn(Storage.prototype, 'removeItem').mockImplementation(
            (key) => {
                delete localStorageMock[key];
            }
        );

        // Create router spy
        routerSpy = {
            navigate: jest.fn().mockResolvedValue(true),
        } as unknown as jest.Mocked<Router>;

        TestBed.configureTestingModule({
            providers: [AuthService, { provide: Router, useValue: routerSpy }],
        });

        service = TestBed.inject(AuthService);
    });

    afterEach(() => {
        jest.restoreAllMocks();
    });

    describe('initialization', () => {
        it('should be created', () => {
            expect(service).toBeTruthy();
        });

        it('should start with no user when no token in storage', () => {
            expect(service.user()).toBeNull();
            expect(service.isAuthenticated()).toBe(false);
        });

        it('should load user from valid stored token', () => {
            const token = createToken(mockUserPayload);
            localStorageMock['mcp_guard_token'] = token;

            // Recreate service to trigger constructor
            TestBed.resetTestingModule();
            TestBed.configureTestingModule({
                providers: [
                    AuthService,
                    { provide: Router, useValue: routerSpy },
                ],
            });
            const newService = TestBed.inject(AuthService);

            expect(newService.user()).not.toBeNull();
            expect(newService.user()?.email).toBe('test@example.com');
            expect(newService.isAuthenticated()).toBe(true);
        });

        it('should clear expired token on initialization', () => {
            const expiredToken = createToken(mockUserPayload, true);
            localStorageMock['mcp_guard_token'] = expiredToken;

            TestBed.resetTestingModule();
            TestBed.configureTestingModule({
                providers: [
                    AuthService,
                    { provide: Router, useValue: routerSpy },
                ],
            });
            const newService = TestBed.inject(AuthService);

            expect(newService.user()).toBeNull();
            expect(newService.isAuthenticated()).toBe(false);
        });
    });

    describe('handleCallback', () => {
        it('should successfully handle valid token', () => {
            const token = createToken(mockUserPayload);

            const result = service.handleCallback(token);

            expect(result).toBe(true);
            expect(service.user()).not.toBeNull();
            expect(service.user()?.email).toBe('test@example.com');
            expect(service.user()?.name).toBe('Test User');
            expect(localStorageMock['mcp_guard_token']).toBe(token);
        });

        it('should reject expired token', () => {
            const expiredToken = createToken(mockUserPayload, true);

            const result = service.handleCallback(expiredToken);

            expect(result).toBe(false);
            expect(service.authError()).toBe('Token has expired');
            expect(service.user()).toBeNull();
        });

        it('should reject invalid token format', () => {
            const result = service.handleCallback('invalid-token');

            expect(result).toBe(false);
            expect(service.authError()).toBe('Invalid token received');
        });

        it('should reject malformed JWT', () => {
            const result = service.handleCallback('a.b');

            expect(result).toBe(false);
            expect(service.authError()).toBe('Invalid token received');
        });
    });

    describe('handleCallbackError', () => {
        it('should set error message', () => {
            service.handleCallbackError('OAuth failed');

            expect(service.authError()).toBe('OAuth failed');
            expect(service.isLoading()).toBe(false);
        });
    });

    describe('logout', () => {
        it('should clear session and navigate to home', () => {
            const token = createToken(mockUserPayload);
            service.handleCallback(token);

            service.logout();

            expect(service.user()).toBeNull();
            expect(service.isAuthenticated()).toBe(false);
            expect(localStorageMock['mcp_guard_token']).toBeUndefined();
            expect(routerSpy.navigate).toHaveBeenCalledWith(['/']);
        });
    });

    describe('isAdmin', () => {
        it('should return false when not authenticated', () => {
            expect(service.isAdmin()).toBe(false);
        });

        it('should return false for regular user', () => {
            const token = createToken(mockUserPayload);
            service.handleCallback(token);

            expect(service.isAdmin()).toBe(false);
        });

        it('should return true for admin user', () => {
            const token = createToken(mockAdminPayload);
            service.handleCallback(token);

            expect(service.isAdmin()).toBe(true);
        });
    });

    describe('hasRole', () => {
        it('should return false when not authenticated', () => {
            expect(service.hasRole('user')).toBe(false);
            expect(service.hasRole('admin')).toBe(false);
        });

        it('should return true for user role for any authenticated user', () => {
            const token = createToken(mockUserPayload);
            service.handleCallback(token);

            expect(service.hasRole('user')).toBe(true);
        });

        it('should return false for admin role for regular user', () => {
            const token = createToken(mockUserPayload);
            service.handleCallback(token);

            expect(service.hasRole('admin')).toBe(false);
        });

        it('should return true for admin role for admin user', () => {
            const token = createToken(mockAdminPayload);
            service.handleCallback(token);

            expect(service.hasRole('admin')).toBe(true);
            expect(service.hasRole('user')).toBe(true);
        });
    });

    describe('getToken', () => {
        it('should return null when no token', () => {
            expect(service.getToken()).toBeNull();
        });

        it('should return stored token', () => {
            const token = createToken(mockUserPayload);
            service.handleCallback(token);

            expect(service.getToken()).toBe(token);
        });
    });

    describe('navigateToDashboard', () => {
        it('should navigate to dashboard', () => {
            service.navigateToDashboard();

            expect(routerSpy.navigate).toHaveBeenCalledWith(['/dashboard']);
        });
    });

    describe('magic link', () => {
        const originalFetch = global.fetch;

        beforeEach(() => {
            global.fetch = jest.fn();
        });

        afterEach(() => {
            global.fetch = originalFetch;
        });

        it('should have magicLinkSent false initially', () => {
            expect(service.magicLinkSent()).toBe(false);
        });

        it('should have magicLinkEmail null initially', () => {
            expect(service.magicLinkEmail()).toBeNull();
        });

        it('should send magic link successfully', async () => {
            (global.fetch as jest.Mock).mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve({ success: true }),
            });

            const result = await service.sendMagicLink('test@example.com');

            expect(result).toBe(true);
            expect(service.magicLinkSent()).toBe(true);
            expect(service.magicLinkEmail()).toBe('test@example.com');
            expect(service.isLoading()).toBe(false);
            expect((global.fetch as jest.Mock)).toHaveBeenCalledWith(
                expect.stringContaining('/auth/magic-link'),
                expect.objectContaining({
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                })
            );
        });

        it('should handle magic link failure', async () => {
            (global.fetch as jest.Mock).mockResolvedValueOnce({
                ok: false,
                json: () => Promise.resolve({ message: 'Email not found' }),
            });

            const result = await service.sendMagicLink('invalid@example.com');

            expect(result).toBe(false);
            expect(service.magicLinkSent()).toBe(false);
            expect(service.authError()).toBe('Email not found');
            expect(service.isLoading()).toBe(false);
        });

        it('should handle network error', async () => {
            (global.fetch as jest.Mock).mockRejectedValueOnce(new Error('Network error'));

            const result = await service.sendMagicLink('test@example.com');

            expect(result).toBe(false);
            expect(service.authError()).toBe('Network error. Please try again.');
            expect(service.isLoading()).toBe(false);
        });

        it('should reset magic link state', async () => {
            (global.fetch as jest.Mock).mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve({ success: true }),
            });
            await service.sendMagicLink('test@example.com');

            service.resetMagicLinkState();

            expect(service.magicLinkSent()).toBe(false);
            expect(service.magicLinkEmail()).toBeNull();
            expect(service.authError()).toBeNull();
        });
    });
});
