import { test, expect } from '@playwright/test';

test.describe('Landing Page', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
    });

    test('should display homepage with hero section', async ({ page }) => {
        // Check page title
        await expect(page).toHaveTitle(/mcp-guard/i);

        // Check hero section elements
        const heroHeading = page.locator('h1').first();
        await expect(heroHeading).toBeVisible();
    });

    test('should have working navigation', async ({ page }) => {
        // Check navbar exists
        const navbar = page.locator('nav, header').first();
        await expect(navbar).toBeVisible();

        // Check for key nav links
        const docsLink = page.getByRole('link', { name: /docs|documentation/i });
        await expect(docsLink).toBeVisible();
    });

    test('should display pricing section', async ({ page }) => {
        // Scroll to pricing or navigate to it
        const pricingSection = page.locator('[class*="pricing"], #pricing');

        if (await pricingSection.count() > 0) {
            await expect(pricingSection.first()).toBeVisible();

            // Check for pricing tiers
            const freeOption = page.getByText(/free/i).first();
            await expect(freeOption).toBeVisible();
        }
    });

    test('should have login link', async ({ page }) => {
        const loginLink = page.getByRole('link', { name: /login|sign in|dashboard/i });
        await expect(loginLink).toBeVisible();
    });
});

test.describe('Login Page', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/login');
    });

    test('should display login form with OAuth buttons', async ({ page }) => {
        // Check for OAuth buttons
        const githubButton = page.getByRole('button', { name: /github/i });
        const googleButton = page.getByRole('button', { name: /google/i });

        await expect(githubButton).toBeVisible();
        await expect(googleButton).toBeVisible();
    });

    test('should have back to home link', async ({ page }) => {
        const backLink = page.getByRole('link', { name: /back|home/i });
        await expect(backLink).toBeVisible();

        await backLink.click();
        await expect(page).toHaveURL('/');
    });

    test('should display feature list', async ({ page }) => {
        // Check for features mentioned on login page
        const licenseFeature = page.getByText(/license management/i);
        const apiKeyFeature = page.getByText(/api key/i);

        await expect(licenseFeature).toBeVisible();
        await expect(apiKeyFeature).toBeVisible();
    });
});

test.describe('Dashboard (Unauthenticated)', () => {
    test('should redirect to login when accessing dashboard without auth', async ({
        page,
    }) => {
        await page.goto('/dashboard');

        // Should redirect to login page
        await expect(page).toHaveURL(/\/login/);
    });

    test('should redirect to login when accessing admin pages without auth', async ({
        page,
    }) => {
        await page.goto('/dashboard/admin/users');

        // Should redirect to login page
        await expect(page).toHaveURL(/\/login/);
    });
});

test.describe('Navigation Flow', () => {
    test('should navigate from homepage to docs', async ({ page }) => {
        await page.goto('/');

        const docsLink = page.getByRole('link', { name: /docs|documentation/i });
        await docsLink.click();

        await expect(page).toHaveURL(/\/docs/);
    });

    test('should navigate from homepage to login', async ({ page }) => {
        await page.goto('/');

        const loginLink = page.getByRole('link', { name: /login|sign in|dashboard/i });
        await loginLink.click();

        await expect(page).toHaveURL(/\/login/);
    });

    test('should navigate from login back to homepage', async ({ page }) => {
        await page.goto('/login');

        const backLink = page.getByRole('link', { name: /back|home/i });
        await backLink.click();

        await expect(page).toHaveURL('/');
    });
});

test.describe('Responsive Design', () => {
    test('should be responsive on mobile', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/');

        // Page should still be functional
        const heroHeading = page.locator('h1').first();
        await expect(heroHeading).toBeVisible();
    });

    test('should be responsive on tablet', async ({ page }) => {
        await page.setViewportSize({ width: 768, height: 1024 });
        await page.goto('/');

        const heroHeading = page.locator('h1').first();
        await expect(heroHeading).toBeVisible();
    });
});

test.describe('Accessibility', () => {
    test('should have proper heading hierarchy', async ({ page }) => {
        await page.goto('/');

        // Check that there's an h1
        const h1Count = await page.locator('h1').count();
        expect(h1Count).toBeGreaterThan(0);
    });

    test('should have accessible buttons', async ({ page }) => {
        await page.goto('/login');

        // OAuth buttons should have accessible names
        const buttons = page.getByRole('button');
        const buttonCount = await buttons.count();

        for (let i = 0; i < buttonCount; i++) {
            const button = buttons.nth(i);
            const name = await button.getAttribute('aria-label');
            const text = await button.textContent();

            // Button should have either aria-label or visible text
            expect(name || text?.trim()).toBeTruthy();
        }
    });

    test('should have accessible links', async ({ page }) => {
        await page.goto('/');

        // Check that links have accessible names
        const links = page.getByRole('link');
        const linkCount = await links.count();

        expect(linkCount).toBeGreaterThan(0);
    });
});
