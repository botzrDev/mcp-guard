import { test, expect } from '@playwright/test';

// Helper to create a dummy JWT
function createMockToken() {
    const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
    const payload = btoa(JSON.stringify({
        sub: '123',
        email: 'test@example.com',
        name: 'Test User',
        role: 'user',
        iat: Math.floor(Date.now() / 1000),
        exp: Math.floor(Date.now() / 1000) + 3600 // 1 hour
    }));
    return `${header}.${payload}.signature`;
}

test.describe('License Flow Validation', () => {

    test('Pricing buttons should lead to signup', async ({ page }) => {
        await page.goto('/');

        // Check "Get Pro" or equivalent buttons
        // Assuming there's a button in the pricing card
        const pricingButtons = page.locator('.pricing-card a, .pricing-card button');
        if (await pricingButtons.count() > 0) {
            const firstButton = pricingButtons.first();
            await expect(firstButton).toBeVisible();
            // Verify it links to signup or similar
            const href = await firstButton.getAttribute('href');
            expect(href).toMatch(/signup|contact/i);
        }
    });

    test.describe('Authenticated Dashboard License View', () => {
        test.beforeEach(async ({ page }) => {
            // Mock authentication
            const token = createMockToken();
            await page.addInitScript((val) => {
                localStorage.setItem('mcp_guard_token', val);
            }, token);

            await page.goto('/dashboard/license');
        });

        test('should display license key section', async ({ page }) => {
            // Verify we are on the license page
            const heading = page.locator('h1');
            await expect(heading).toHaveText('License');

            // Verify License Key section exists
            const keySection = page.locator('.license-key-section');
            await expect(keySection).toBeVisible();
        });

        test('license key should be masked by default', async ({ page }) => {
            const codeBlock = page.locator('.key-value');
            const text = await codeBlock.textContent();
            expect(text).toMatch(/\.\.\./); // Should contain ellipsis
            expect(text?.length).toBeLessThan(50); // Should be short/masked
        });

        test('toggle button should reveal/hide key', async ({ page }) => {
            const codeBlock = page.locator('.key-value');
            const toggleBtn = page.locator('.key-actions button').first(); // First button is usually toggle (eye)

            // It's masked initially
            await expect(codeBlock).toContainText('...');

            // Click to show
            await toggleBtn.click();

            // Check if full key is shown (no ellipsis in the middle, or long enough)
            // The demo key is "pro_demo_key_abc123xyz789"
            await expect(codeBlock).toContainText('pro_demo_key');
            await expect(codeBlock).not.toContainText('...');

            // Click to hide
            await toggleBtn.click();
            await expect(codeBlock).toContainText('...');
        });

        test('copy button should provide feedback', async ({ page, context }) => {
            // Grant clipboard permissions
            await context.grantPermissions(['clipboard-read', 'clipboard-write']);

            const copyBtn = page.locator('button[title="Copy"]');
            await expect(copyBtn).toBeVisible();

            await copyBtn.click();

            // Check for feedback "Copied to clipboard!"
            const feedback = page.locator('.copy-feedback');
            await expect(feedback).toBeVisible();
            await expect(feedback).toHaveText(/copied/i);
        });

        test('usage instructions should be present', async ({ page }) => {
            const instructions = page.locator('.usage-instructions');
            await expect(instructions).toBeVisible();

            // Check for specific instruction blocks
            await expect(page.getByText('Environment Variable')).toBeVisible();
            await expect(page.getByText('Docker')).toBeVisible();
            await expect(page.getByText('systemd Service')).toBeVisible();
        });
    });
});
