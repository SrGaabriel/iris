mod server;
mod schema;
mod database;
mod util;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use argon2::Argon2;
use argon2::password_hash::SaltString;
use axum::{extract::ws::{Message, WebSocket, WebSocketUpgrade}, response::IntoResponse, routing::get, Router, middleware};
use axum::body::Body;
use axum::middleware::AddExtension;
use axum::routing::{delete, patch, post, put};
use crossbeam::queue::SegQueue;
use dashmap::DashMap;
use diesel::PgConnection;
use dotenvy::dotenv;
use futures_util::SinkExt;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tower_http::add_extension::AddExtensionLayer;
use tower_http::auth::{AsyncRequireAuthorizationLayer, AsyncAuthorizeRequest};
use tower_http::cors::CorsLayer;
use tracing::Subscriber;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::schema::users::User;
use crate::server::gateway::Gateway;
use crate::server::messages::Packet;
use crate::server::rest::middlewares::{authorize, IrisAuth};
use crate::util::snowflake::SnowflakeIssuer;

#[tokio::main]
async fn main() {
    dotenv().ok().expect("Failed to read .env file");
    let key = Hmac::<Sha256>::new_from_slice(std::env::var("JWT_SECRET").expect("JWT token could not be read").as_bytes())
        .expect("Failed to create HMAC");
    let salt = SaltString::from_b64(&*std::env::var("ARGON_SALT").expect("Argon salt could not be read"))
        .expect("Failed to create salt");

    let database_connection = database::connect();
    let mut gateway = Gateway::new();
    gateway.register_handler(Box::new(server::gateway::receipts::ReceiptGatewayHandler));
    gateway.register_handler(Box::new(server::gateway::typing::TypingGatewayHandler));

    let state = AppState {
        gateway,
        packet_queue: DashMap::new(),
        database: database_connection,
        jwt_key: key,
        argon: Argon2::default(),
        argon_salt: salt,
        snowflake_issuer: SnowflakeIssuer::new(1,1)
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=info,diesel=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/ws", get(server::subscribe_chat_handshake))
        .route("/api/users/@me", get(server::rest::user::get_self))
        .route("/api/contacts/@me", get(server::rest::contacts::get_contacts))
        .route("/api/contacts/:contact_id", get(server::rest::contacts::get_contact))
        .route("/api/channels/:channel_id/messages", post(server::rest::messages::create_message))
        .route("/api/channels/:channel_id/messages", get(server::rest::messages::get_messages))
        .route("/api/channels/:channel_id/messages/:message_id", put(server::rest::messages::edit_message))
        .route("/api/channels/:channel_id/messages/:message_id", delete(server::rest::messages::delete_message))
        .route("/api/channels/:channel_id/messages/:message_id/reactions", post(server::rest::reactions::add_reaction))
        .route("/api/channels/:channel_id/messages/:message_id/reactions/:reaction_id", delete(server::rest::reactions::remove_reaction))
        .route_layer(
            middleware::from_fn(authorize)
        )
        .route("/login", post(server::rest::auth::login))
        .route("/signup", post(server::rest::auth::register))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true))
                )
                .layer(AddExtensionLayer::new(Arc::new(RwLock::new(state))))
                .into_inner()
        );

    let listener = TcpListener::bind("127.0.0.1:3000")
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
    pub gateway: Gateway,
    pub packet_queue: DashMap<i64, Sender<Box<dyn Packet + Send>>>,
    pub database: PgConnection,
    pub jwt_key: Hmac<Sha256>,
    pub argon: Argon2<'static>,
    pub argon_salt: SaltString,
    pub snowflake_issuer: SnowflakeIssuer
}

unsafe impl Sync for AppState {}
unsafe impl Send for AppState {}

pub type SharedState = Arc<RwLock<AppState>>;