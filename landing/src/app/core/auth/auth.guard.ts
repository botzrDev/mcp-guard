import { inject } from '@angular/core';
import { Router, CanActivateFn, ActivatedRouteSnapshot } from '@angular/router';
import { AuthService } from './auth.service';
import { UserRole } from './auth.models';

export const AuthGuard: CanActivateFn = () => {
    const authService = inject(AuthService);
    const router = inject(Router);

    if (authService.isAuthenticated()) {
        return true;
    }

    router.navigate(['/login'], {
        queryParams: { returnUrl: window.location.pathname }
    });
    return false;
};

export const RoleGuard: CanActivateFn = (route: ActivatedRouteSnapshot) => {
    const authService = inject(AuthService);
    const router = inject(Router);

    if (!authService.isAuthenticated()) {
        router.navigate(['/login']);
        return false;
    }

    const requiredRoles = route.data['roles'] as UserRole[] | undefined;
    if (!requiredRoles || requiredRoles.length === 0) {
        return true;
    }

    const hasRequiredRole = requiredRoles.some(role => authService.hasRole(role));
    if (hasRequiredRole) {
        return true;
    }

    router.navigate(['/dashboard']);
    return false;
};

export const GuestGuard: CanActivateFn = () => {
    const authService = inject(AuthService);
    const router = inject(Router);

    if (!authService.isAuthenticated()) {
        return true;
    }

    router.navigate(['/dashboard']);
    return false;
};
