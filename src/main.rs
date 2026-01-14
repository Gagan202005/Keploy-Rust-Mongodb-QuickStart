use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bson::{doc, Document};
use dotenvy::dotenv;
use futures::stream::StreamExt;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    collection: Collection<Document>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoteRequest {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoteResponse {
    text: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_uri = std::env::var("MONGODB_URI")
    .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let db_name = env::var("DB_NAME").unwrap_or("keploydb".to_string());
    let collection_name = env::var("COLLECTION_NAME").unwrap_or("notes".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());

    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("❌ Failed to connect to MongoDB");

    let db = client.database(&db_name);
    let collection = db.collection::<Document>(&collection_name);

    let state = AppState { collection };

    let app = Router::new()
        .route("/", get(root))
        .route("/notes", post(create_note).get(get_notes))
        .with_state(state);

    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running at http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "🚀 Rust + MongoDB + Keploy Quickstart is running!"
}

async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<NoteRequest>,
) -> Result<(StatusCode, Json<NoteResponse>), (StatusCode, String)> {
    let doc = doc! { "text": &payload.text };

    state
        .collection
        .insert_one(doc)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB insert error: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        Json(NoteResponse { text: payload.text }),
    ))
}

async fn get_notes(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<NoteResponse>>), (StatusCode, String)> {
    let mut cursor = state
        .collection
        .find(doc! {})
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB find error: {}", e)))?;

    let mut notes: Vec<NoteResponse> = vec![];

    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                if let Some(text) = doc.get_str("text").ok() {
                    notes.push(NoteResponse {
                        text: text.to_string(),
                    });
                }
            }
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Cursor error: {}", e)))
            }
        }
    }

    Ok((StatusCode::OK, Json(notes)))
}
