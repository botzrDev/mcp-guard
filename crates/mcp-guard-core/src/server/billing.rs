use axum::{
    extract::{State},
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use stripe::{CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionSubscriptionData};

use crate::server::{AppError, AppState};

#[derive(Deserialize)]
pub struct CreateCheckoutSessionRequest {
    email: String,
    price_id: String,
    success_url: String,
    cancel_url: String,
}

#[derive(Serialize)]
pub struct CreateCheckoutSessionResponse {
    session_id: String,
    url: String,
}

pub async fn create_checkout_session(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCheckoutSessionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let secret_key = state.config.stripe_secret_key.as_ref().ok_or_else(|| {
        tracing::error!("Stripe secret key not configured");
        AppError::internal("Server configuration error")
    })?;

    let client = Client::new(secret_key);

    let checkout_session = CheckoutSession::create(
        &client,
        CreateCheckoutSession {
            mode: Some(CheckoutSessionMode::Subscription),
            customer_email: Some(&payload.email),
            line_items: Some(vec![CreateCheckoutSessionLineItems {
                price: Some(payload.price_id),
                quantity: Some(1),
                ..Default::default()
            }]),
            success_url: Some(&payload.success_url),
            cancel_url: Some(&payload.cancel_url),
            subscription_data: Some(CreateCheckoutSessionSubscriptionData {
                trial_period_days: Some(7),
                ..Default::default()
            }),
            allow_promotion_codes: Some(true),
            ..Default::default()
        },
    ).await.map_err(|e| {
        tracing::error!("Stripe error: {}", e);
        AppError::internal("Failed to create checkout session")
    })?;

    let url = checkout_session.url.ok_or_else(|| {
        tracing::error!("No checkout URL returned from Stripe");
        AppError::internal("Failed to create checkout session")
    })?;

    Ok(Json(CreateCheckoutSessionResponse {
        session_id: checkout_session.id.as_str().to_string(),
        url,
    }))
}
