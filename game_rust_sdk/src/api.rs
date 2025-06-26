use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::{GameTokenSdk, error::SdkError};

/// Represents the API state/context
pub struct ApiContext {}

impl ApiContext {
    pub fn new() -> Self {
        Self {}
    }
}

/// Start the API server
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Arc::new(ApiContext::new());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router with all routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/v1/calldata/creda-approve", post(approve_creda))
        .route("/api/v1/calldata/approve-xp", post(approve_xp))
        .route("/api/v1/calldata/lock-creda", post(lock_creda))
        .route("/api/v1/calldata/create-token", post(create_game_token))
        .route("/api/v1/calldata/burn-token", post(burn_game_token))
        .route("/api/v1/flow/create", post(create_token_flow))
        .route("/api/v1/flow/burn", post(burn_token_flow))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(ctx);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("API server listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// API error type
#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    status_code: u16,
    error_type: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(self);
        (status, body).into_response()
    }
}

impl From<SdkError> for ApiError {
    fn from(err: SdkError) -> Self {
        match err {
            SdkError::InvalidAddress(msg) => ApiError {
                message: format!("Invalid Ethereum address: {}", msg),
                status_code: 400,
                error_type: "INVALID_ADDRESS".into(),
            },
            SdkError::InvalidNumber(msg) => ApiError {
                message: format!("Invalid number format: {}", msg),
                status_code: 400,
                error_type: "INVALID_NUMBER".into(),
            },
            _ => ApiError {
                message: format!("SDK error: {}", err),
                status_code: 500,
                error_type: "SDK_ERROR".into(),
            },
        }
    }
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    #[derive(Serialize)]
    struct HealthResponse {
        status: &'static str,
        version: &'static str,
    }

    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

// ============== API Endpoints ===============

// --- Request and Response Types ---

#[derive(Debug, Deserialize)]
pub struct ApproveRequest {
    pub spender: String,
    pub amount: String,
}

#[derive(Debug, Serialize)]
pub struct CallDataResponse {
    pub calldata: String,
}

#[derive(Debug, Deserialize)]
pub struct AmountRequest {
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    pub xp_amount: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Deserialize)]
pub struct BurnTokenRequest {
    pub game_id: String,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFlowRequest {
    pub factory_address: String,
    pub creda_amount: String,
    pub game_name: String,
    pub game_symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize)]
pub struct CreateFlowResponse {
    pub creda_approve: String,
    pub lock_creda: String,
    pub xp_amount: String,
    pub xp_approve: String,
    pub create_token: String,
}

#[derive(Debug, Deserialize)]
pub struct BurnFlowRequest {
    pub factory_address: String,
    pub game_token_address: String,
    pub game_id: String,
    pub burn_amount: String,
}

#[derive(Debug, Serialize)]
pub struct BurnFlowResponse {
    pub game_token_approve: String,
    pub burn_game_token: String,
}

// --- Handler Functions ---

// Approve CRIDA tokens
async fn approve_creda(
    State(_ctx): State<Arc<ApiContext>>,
    Json(payload): Json<ApproveRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Generate the calldata
    let calldata = GameTokenSdk::approve_crida(&payload.spender, &payload.amount)?;
    
    Ok(Json(CallDataResponse { calldata }))
}

// Approve XP tokens
async fn approve_xp(
    State(_ctx): State<Arc<ApiContext>>,
    Json(payload): Json<ApproveRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Generate the calldata
    let calldata = GameTokenSdk::approve_xp(&payload.spender, &payload.amount)?;
    
    Ok(Json(CallDataResponse { calldata }))
}

// Lock CRIDA tokens
async fn lock_creda(
    State(_ctx): State<Arc<ApiContext>>,
    Json(payload): Json<AmountRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Generate the calldata
    let calldata = GameTokenSdk::lock_crida(&payload.amount)?;
    
    Ok(Json(CallDataResponse { calldata }))
}

// Create a game token
async fn create_game_token(
    State(_ctx): State<Arc<ApiContext>>,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Generate the calldata
    let calldata = GameTokenSdk::create_game_token(
        &payload.xp_amount,
        &payload.name,
        &payload.symbol,
        payload.decimals,
    )?;
    
    Ok(Json(CallDataResponse { calldata }))
}

// Burn a game token
async fn burn_game_token(
    State(_ctx): State<Arc<ApiContext>>,
    Json(payload): Json<BurnTokenRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Generate the calldata
    let calldata = GameTokenSdk::burn_game_token(&payload.game_id, &payload.amount)?;
    
    Ok(Json(CallDataResponse { calldata }))
}

// Complete create token flow
async fn create_token_flow(
    State(_ctx): State<Arc<ApiContext>>,
    Json(payload): Json<CreateFlowRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Generate the calldata for the complete flow
    let flow = GameTokenSdk::create_game_token_flow(
        &payload.factory_address,
        &payload.creda_amount,
        &payload.game_name,
        &payload.game_symbol,
        payload.decimals,
    )?;
    
    let response = CreateFlowResponse {
        creda_approve: flow.creda_approve,
        lock_creda: flow.lock_creda,
        xp_amount: flow.xp_amount.to_string(),
        xp_approve: flow.xp_approve,
        create_token: flow.create_token,
    };
    
    Ok(Json(response))
}

// Complete burn token flow
async fn burn_token_flow(
    State(_ctx): State<Arc<ApiContext>>,
    Json(payload): Json<BurnFlowRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Generate the calldata for the burn flow
    let flow = GameTokenSdk::burn_game_token_flow(
        &payload.factory_address,
        &payload.game_token_address,
        &payload.game_id,
        &payload.burn_amount,
    )?;
    
    let response = BurnFlowResponse {
        game_token_approve: flow.game_token_approve,
        burn_game_token: flow.burn_game_token,
    };
    
    Ok(Json(response))
} 