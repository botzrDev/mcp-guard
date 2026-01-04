import { TestBed } from '@angular/core/testing';
import { Router, ActivatedRouteSnapshot, RouterStateSnapshot } from '@angular/router';
import { AuthService } from './auth.service';
import { AuthGuard, RoleGuard, GuestGuard } from './auth.guard';

describe('Auth Guards', () => {
    let authServiceMock: jest.Mocked<Partial<AuthService>>;
    let routerMock: jest.Mocked<Router>;

    beforeEach(() => {
        authServiceMock = {
            isAuthenticated: jest.fn(),
            hasRole: jest.fn(),
        };

        routerMock = {
            navigate: jest.fn().mockResolvedValue(true),
        } as unknown as jest.Mocked<Router>;

        TestBed.configureTestingModule({
            providers: [
                { provide: AuthService, useValue: authServiceMock },
                { provide: Router, useValue: routerMock },
            ],
        });
    });

    afterEach(() => {
        jest.clearAllMocks();
    });

    describe('AuthGuard', () => {
        it('should allow access when authenticated', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(true);

            const result = TestBed.runInInjectionContext(() =>
                AuthGuard({} as ActivatedRouteSnapshot, {} as RouterStateSnapshot)
            );

            expect(result).toBe(true);
            expect(routerMock.navigate).not.toHaveBeenCalled();
        });

        it('should redirect to login when not authenticated', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(false);

            const result = TestBed.runInInjectionContext(() =>
                AuthGuard({} as ActivatedRouteSnapshot, {} as RouterStateSnapshot)
            );

            expect(result).toBe(false);
            expect(routerMock.navigate).toHaveBeenCalledWith(['/login'], {
                queryParams: { returnUrl: expect.any(String) },
            });
        });
    });

    describe('RoleGuard', () => {
        const createRouteSnapshot = (roles?: string[]): ActivatedRouteSnapshot => {
            return {
                data: roles ? { roles } : {},
            } as unknown as ActivatedRouteSnapshot;
        };

        it('should redirect to login when not authenticated', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(false);
            const route = createRouteSnapshot(['admin']);

            const result = TestBed.runInInjectionContext(() =>
                RoleGuard(route, {} as RouterStateSnapshot)
            );

            expect(result).toBe(false);
            expect(routerMock.navigate).toHaveBeenCalledWith(['/login']);
        });

        it('should allow access when no roles are required', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(true);
            const route = createRouteSnapshot();

            const result = TestBed.runInInjectionContext(() =>
                RoleGuard(route, {} as RouterStateSnapshot)
            );

            expect(result).toBe(true);
            expect(routerMock.navigate).not.toHaveBeenCalled();
        });

        it('should allow access when empty roles array', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(true);
            const route = createRouteSnapshot([]);

            const result = TestBed.runInInjectionContext(() =>
                RoleGuard(route, {} as RouterStateSnapshot)
            );

            expect(result).toBe(true);
        });

        it('should allow access when user has required role', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(true);
            authServiceMock.hasRole!.mockImplementation((role) => role === 'admin');
            const route = createRouteSnapshot(['admin']);

            const result = TestBed.runInInjectionContext(() =>
                RoleGuard(route, {} as RouterStateSnapshot)
            );

            expect(result).toBe(true);
            expect(authServiceMock.hasRole).toHaveBeenCalledWith('admin');
        });

        it('should redirect to dashboard when user lacks required role', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(true);
            authServiceMock.hasRole!.mockReturnValue(false);
            const route = createRouteSnapshot(['admin']);

            const result = TestBed.runInInjectionContext(() =>
                RoleGuard(route, {} as RouterStateSnapshot)
            );

            expect(result).toBe(false);
            expect(routerMock.navigate).toHaveBeenCalledWith(['/dashboard']);
        });

        it('should allow access if user has any of the required roles', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(true);
            authServiceMock.hasRole!.mockImplementation(
                (role) => role === 'moderator'
            );
            const route = createRouteSnapshot(['admin', 'moderator']);

            const result = TestBed.runInInjectionContext(() =>
                RoleGuard(route, {} as RouterStateSnapshot)
            );

            expect(result).toBe(true);
        });
    });

    describe('GuestGuard', () => {
        it('should allow access when not authenticated', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(false);

            const result = TestBed.runInInjectionContext(() =>
                GuestGuard({} as ActivatedRouteSnapshot, {} as RouterStateSnapshot)
            );

            expect(result).toBe(true);
            expect(routerMock.navigate).not.toHaveBeenCalled();
        });

        it('should redirect to dashboard when authenticated', () => {
            authServiceMock.isAuthenticated!.mockReturnValue(true);

            const result = TestBed.runInInjectionContext(() =>
                GuestGuard({} as ActivatedRouteSnapshot, {} as RouterStateSnapshot)
            );

            expect(result).toBe(false);
            expect(routerMock.navigate).toHaveBeenCalledWith(['/dashboard']);
        });
    });
});
