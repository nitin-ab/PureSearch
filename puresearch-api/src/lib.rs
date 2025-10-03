use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use puresearch_core::{ReviewDocument, Index};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use puresearch_storage::MmapStorage;
use puresearch_core::storage::{StorageEngine, IndexStorage};

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
    let storage = Arc::new(Mutex::new(MmapStorage::new("./data").unwrap()));

    Router::new()
        .route("/health", get(health_check))
        .route("/documents", post(ingest_document))
        .route("/documents/:id", get(get_document))
        .route("/search", get(search_documents))
        .route("/indices", post(create_index))
        .route("/indices", get(list_indices))
        .with_state(storage)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn ingest_document(
    State(state): State<Arc<Mutex<MmapStorage>>>,
    Json(req): Json<DocumentRequest>,
) -> Result<Json<ReviewDocument>, StatusCode> {
    let mut storage = state.lock().unwrap();
    let doc = ReviewDocument::new(
        req.content,
        req.metadata.unwrap_or_default(),
    );
    storage.store_document(&doc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(doc))
}

async fn get_document(
    State(state): State<Arc<Mutex<MmapStorage>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ReviewDocument>, StatusCode> {
    let storage = state.lock().unwrap();
    let doc = storage.get_document(&id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    doc.map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn search_documents(
    State(state): State<Arc<Mutex<MmapStorage>>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let storage = state.lock().unwrap();
    let doc_ids = storage.list_documents().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut documents = vec![];
    let limit = query.limit.unwrap_or(10);
    
    for id in doc_ids {
        if let Some(doc) = storage.get_document(&id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
            if doc.content.to_lowercase().contains(&query.q.to_lowercase()) {
                documents.push(doc);
                if documents.len() >= limit {
                    break;
                }
            }
        }
    }
    
    let response = SearchResponse {
        total: documents.len(),
        documents,
    };
    Ok(Json(response))
}

async fn create_index(
    State(state): State<Arc<Mutex<MmapStorage>>>,
    Json(name): Json<String>,
) -> Result<Json<Index>, StatusCode> {
    let mut storage = state.lock().unwrap();
    let index = Index::new(name);
    storage.store_index(&index).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(index))
}

async fn list_indices(
    State(state): State<Arc<Mutex<MmapStorage>>>,
) -> Result<Json<Vec<Index>>, StatusCode> {
    let storage = state.lock().unwrap();
    let indices = storage.list_indices().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(indices))
}
