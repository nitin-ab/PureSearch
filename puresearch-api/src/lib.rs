use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use puresearch_core::{ReviewDocument, Index};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub documents: Vec<ReviewDocument>,
    pub total: usize,
}

#[derive(Deserialize)]
pub struct DocumentRequest {
    pub content: String,
    pub metadata: Option<HashMap<String, String>>,
}

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/documents", post(ingest_document))
        .route("/documents/:id", get(get_document))
        .route("/search", get(search_documents))
        .route("/indices", post(create_index))
        .route("/indices", get(list_indices))
}

async fn health_check() -> &'static str {
    "OK"
}

async fn ingest_document(
    Json(req): Json<DocumentRequest>,
) -> Result<Json<ReviewDocument>, StatusCode> {
    let doc = ReviewDocument::new(
        req.content,
        req.metadata.unwrap_or_default(),
    );
    
    Ok(Json(doc))
}

async fn get_document(Path(id): Path<Uuid>) -> Result<Json<ReviewDocument>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn search_documents(
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let response = SearchResponse {
        documents: vec![],
        total: 0,
    };
    
    Ok(Json(response))
}

async fn create_index(Json(name): Json<String>) -> Result<Json<Index>, StatusCode> {
    let index = Index::new(name);
    Ok(Json(index))
}

async fn list_indices() -> Result<Json<Vec<Index>>, StatusCode> {
    Ok(Json(vec![]))
}
