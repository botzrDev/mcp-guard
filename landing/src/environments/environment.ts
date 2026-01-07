// ============================================================================
// Development Environment Configuration
// ============================================================================
// This file is for LOCAL DEVELOPMENT ONLY
// Just paste your Stripe TEST keys below (get from Stripe Dashboard)
// ============================================================================

export const environment = {
    production: false,

    // ========================================================================
    // >>> PASTE YOUR STRIPE TEST KEYS HERE <<<
    // ========================================================================
    // Get from: https://dashboard.stripe.com/test/apikeys
    //
    // 1. Publishable key (starts with pk_test_...)
    // 2. Price ID from Products (starts with price_...)
    //
    stripePublishableKey: 'pk_test_PASTE_YOUR_TEST_PUBLISHABLE_KEY_HERE',
    stripePriceId: 'price_PASTE_YOUR_TEST_PRICE_ID_HERE',

    // Backend API URL (leave as-is for local development)
    apiUrl: 'http://localhost:3000'
};
