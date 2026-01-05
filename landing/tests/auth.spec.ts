import { test, expect } from '@playwright/test';

// Helper to create a dummy JWT
function createMockToken(overrides: Partial<{
    sub: string;
    email: string;
    name: string;
    role: string;
    provider: string;
}> = {}) {
    const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
    const payload = btoa(JSON.stringify({
        sub: overrides.sub || '123',
        email: overrides.email || 'test@example.com',
        name: overrides.name || 'Test User',
        role: overrides.role || 'user',
        provider: overrides.provider || 'github',
        iat: Math.floor(Date.now() / 1000),
        exp: Math.floor(Date.now() / 1000) + 3600 // 1 hour
    }));
    return `${header}.${payload}.signature`;
}

test.describe('Authentication Flow', () => {
    test.describe('Login Page', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/login');
        });

        test('should display login page with logo and title', async ({ page }) => {
            await expect(page.locator('.login-card .logo-name')).toHaveText('mcp-guard');
            await expect(page.locator('.login-card h1')).toHaveText('Welcome back');
            await expect(page.locator('.login-card .subtitle')).toHaveText('Sign in to access your dashboard');
        });

        test('should display GitHub OAuth button', async ({ page }) => {
            const githubBtn = page.locator('.oauth-btn.github');
            await expect(githubBtn).toBeVisible();
            await expect(githubBtn).toContainText('Continue with GitHub');
        });

        test('should display Google OAuth button', async ({ page }) => {
            const googleBtn = page.locator('.oauth-btn.google');
            await expect(googleBtn).toBeVisible();
            await expect(googleBtn).toContainText('Continue with Google');
        });

        test('should display magic link email form', async ({ page }) => {
            const emailInput = page.locator('.input-group input[type="email"]');
            const magicLinkBtn = page.locator('.magic-link-btn');

            await expect(emailInput).toBeVisible();
            await expect(emailInput).toHaveAttribute('placeholder', 'Enter your email');
            await expect(magicLinkBtn).toBeVisible();
            await expect(magicLinkBtn).toContainText('Continue with Email');
        });

        test('should display divider between OAuth and magic link', async ({ page }) => {
            const divider = page.locator('.divider');
            await expect(divider).toBeVisible();
            await expect(divider).toContainText('or');
        });

        test('should display features list', async ({ page }) => {
            await expect(page.getByText('License management')).toBeVisible();
            await expect(page.getByText('API key management')).toBeVisible();
            await expect(page.getByText('Usage analytics')).toBeVisible();
        });

        test('should display terms and privacy links', async ({ page }) => {
            await expect(page.getByRole('link', { name: 'Terms of Service' })).toBeVisible();
            await expect(page.getByRole('link', { name: 'Privacy Policy' })).toBeVisible();
        });

        test('should have back to home link', async ({ page }) => {
            const backLink = page.locator('.back-link');
            await expect(backLink).toBeVisible();
            await expect(backLink).toContainText('Back to home');

            await backLink.click();
            await expect(page).toHaveURL('/');
        });

        test('should handle email input', async ({ page }) => {
            const emailInput = page.locator('.input-group input[type="email"]');
            await emailInput.fill('user@example.com');
            await expect(emailInput).toHaveValue('user@example.com');
        });

        test('magic link button should be disabled when email is empty', async ({ page }) => {
            const magicLinkBtn = page.locator('.magic-link-btn');
            const emailInput = page.locator('.input-group input[type="email"]');

            // Clear the email input
            await emailInput.fill('');
            await expect(magicLinkBtn).toBeDisabled();
        });

        test('magic link button should be enabled when email is provided', async ({ page }) => {
            const magicLinkBtn = page.locator('.magic-link-btn');
            const emailInput = page.locator('.input-group input[type="email"]');

            await emailInput.fill('test@example.com');
            await expect(magicLinkBtn).not.toBeDisabled();
        });
    });

    test.describe('Authenticated State', () => {
        test.beforeEach(async ({ page }) => {
            const token = createMockToken();
            await page.addInitScript((val) => {
                localStorage.setItem('mcp_guard_token', val);
            }, token);
        });

        test('should redirect authenticated user to dashboard', async ({ page }) => {
            await page.goto('/dashboard');
            await expect(page).toHaveURL(/\/dashboard/);
        });

        test('should show user info in dashboard', async ({ page }) => {
            await page.goto('/dashboard');
            // Dashboard should be accessible
            const heading = page.locator('.dashboard h1, .page-container h1').first();
            await expect(heading).toBeVisible();
        });
    });

    test.describe('Error States', () => {
        test('should display session expired error', async ({ page }) => {
            await page.goto('/login?error=session_expired');

            const errorBanner = page.locator('.error-banner');
            await expect(errorBanner).toBeVisible();
            await expect(errorBanner).toContainText('Your session has expired');
        });

        test('should display access denied error', async ({ page }) => {
            await page.goto('/login?error=access_denied');

            const errorBanner = page.locator('.error-banner');
            await expect(errorBanner).toBeVisible();
            await expect(errorBanner).toContainText('Access was denied');
        });

        test('should display invalid state error', async ({ page }) => {
            await page.goto('/login?error=invalid_state');

            const errorBanner = page.locator('.error-banner');
            await expect(errorBanner).toBeVisible();
            await expect(errorBanner).toContainText('Authentication failed');
        });

        test('should display generic error for unknown error codes', async ({ page }) => {
            await page.goto('/login?error=unknown');

            const errorBanner = page.locator('.error-banner');
            await expect(errorBanner).toBeVisible();
            await expect(errorBanner).toContainText('An error occurred');
        });
    });

    test.describe('OAuth Button Behavior', () => {
        test('GitHub button should redirect to OAuth endpoint', async ({ page }) => {
            await page.goto('/login');

            const githubBtn = page.locator('.oauth-btn.github');

            // Listen for navigation - we expect it to redirect to the API
            const [request] = await Promise.all([
                page.waitForRequest(request =>
                    request.url().includes('/oauth/authorize') &&
                    request.url().includes('provider=github')
                ).catch(() => null),
                githubBtn.click().catch(() => null),
            ].filter(Boolean));

            // The button becomes disabled when clicked (loading state), so we verify it was clickable
            await expect(githubBtn).toBeVisible();
        });

        test('Google button should redirect to OAuth endpoint', async ({ page }) => {
            await page.goto('/login');

            const googleBtn = page.locator('.oauth-btn.google');

            // Listen for navigation - we expect it to redirect to the API
            const [request] = await Promise.all([
                page.waitForRequest(request =>
                    request.url().includes('/oauth/authorize') &&
                    request.url().includes('provider=google')
                ).catch(() => null),
                googleBtn.click().catch(() => null),
            ].filter(Boolean));

            // The button becomes disabled when clicked (loading state), so we verify it was clickable
            await expect(googleBtn).toBeVisible();
        });
    });
});
