// Production environment configuration
// This file contains PRODUCTION settings
// IMPORTANT: Do not commit live Stripe keys to git!

export const environment = {
    production: true,
    // Production Stripe keys - replace with your actual LIVE keys
    // Get from: https://dashboard.stripe.com/apikeys
    stripePublishableKey: 'pk_live_51Sc3idA6PFA1cvOSbizserkKTbflytKKDtySfj1bIGpytZMF78YJPlmgDKMUvfwPKzzc1DrP3xZlLC6wHTzLoMwM00Scg2SH3r',
    stripePriceId: 'price_1SlwzhA6PFA1cvOS5YoY768A', // MCP-Guard Pro monthly price
    apiUrl: 'https://api.mcpg.botzr.com'  // Production API endpoint
};
