import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
    });

    test('should navigate to docs', async ({ page }) => {
        await page.getByText('Docs').first().click();
        await expect(page).toHaveURL(/\/docs/);
        await expect(page.locator('h3')).toContainText('Documentation');
    });

    test('should navigate to changelog', async ({ page }) => {
        // Navigate via footer or direct URL since it might be in utility bar
        await page.goto('/changelog');
        await expect(page).toHaveURL(/\/changelog/);
        await expect(page.locator('h1')).toContainText('Changelog');
    });

    test('should navigate to quickstart', async ({ page }) => {
        await page.goto('/docs/quickstart');
        await expect(page).toHaveURL(/\/docs\/quickstart/);
        await expect(page.locator('h1').first()).toContainText('Quick Start');
    });

    test('should scroll to features', async ({ page }) => {
        // Click the features link
        await page.locator('a[href="/#features"]').first().click();
        // Verify URL fragment
        await expect(page).toHaveURL(/#features/);
    });

    test('should show home page content', async ({ page }) => {
        await expect(page.locator('app-hero')).toBeVisible();
    });
});
