mod server;
mod entity;
mod database;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use argon2::Argon2;
use argon2::password_hash::SaltString;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::middleware::AddExtension;
use axum::routing::post;
use diesel::PgConnection;
use dotenvy::dotenv;
use futures_util::SinkExt;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::CorsLayer;
use tracing::Subscriber;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::entity::user::User;

#[tokio::main]
async fn main() {
    dotenv().ok().expect("Failed to read .env file");
    let key = Hmac::<Sha256>::new_from_slice(std::env::var("JWT_SECRET").expect("JWT token could not be read").as_bytes())
        .expect("Failed to create HMAC");
    let salt = SaltString::from_b64(&*std::env::var("ARGON_SALT").expect("Argon salt could not be read"))
        .expect("Failed to create salt");

    let database_connection = database::connect();
    let state = AppState {
        connections: HashMap::new(),
        database: database_connection,
        jwt_key: key,
        argon: Argon2::default(),
        argon_salt: salt
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/login", post(server::rest::auth::login))
        .route("/signup", post(server::rest::auth::register))
        .route("/ws", get(server::subscribe_chat_handshake))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true))
                )
                .layer(AddExtensionLayer::new(Arc::new(RwLock::new(state))))
                .into_inner()
        );

    let listener = TcpListener::bind("127.0.0.1:7989")
        .await
        .expect("failed to bind to address");
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("failed to run server");
}

pub struct AppState {
    pub connections: HashMap<SocketAddr, User>,
    pub database: PgConnection,
    pub jwt_key: Hmac<Sha256>,
    pub argon: Argon2<'static>,
    pub argon_salt: SaltString
}

unsafe impl Sync for AppState {}
unsafe impl Send for AppState {}

pub type SharedState = Arc<RwLock<AppState>>;