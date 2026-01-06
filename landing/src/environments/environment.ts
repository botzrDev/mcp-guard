// Development environment configuration
// This file is for LOCAL DEVELOPMENT ONLY
// Use TEST keys from Stripe Dashboard: https://dashboard.stripe.com/test/apikeys

export const environment = {
    production: false,
    // IMPORTANT: Use Stripe TEST keys (pk_test_...) for development
    // Get test keys from: https://dashboard.stripe.com/test/apikeys
    stripePublishableKey: 'pk_test_YOUR_TEST_KEY_HERE', // Replace with pk_test_... key
    stripePriceId: 'price_test_YOUR_TEST_PRICE_HERE',   // Replace with test price ID
    apiUrl: 'http://localhost:3000'
};
