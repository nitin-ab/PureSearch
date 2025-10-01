use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDocument {
    pub id: Uuid,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub timestamp: u64,
}

impl ReviewDocument {
    pub fn new(content: String, metadata: HashMap<String, String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            metadata,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub id: Uuid,
    pub name: String,
    pub documents: Vec<Uuid>,
    pub created_at: u64,
}

impl Index {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            documents: Vec::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn add_document(&mut self, doc_id: Uuid) {
        if !self.documents.contains(&doc_id) {
            self.documents.push(doc_id);
        }
    }
}

pub mod storage {
    use super::*;
    use anyhow::Result;

    pub trait StorageEngine {
        fn store_document(&mut self, doc: &ReviewDocument) -> Result<()>;
        fn get_document(&self, id: &Uuid) -> Result<Option<ReviewDocument>>;
        fn delete_document(&mut self, id: &Uuid) -> Result<bool>;
        fn list_documents(&self) -> Result<Vec<Uuid>>;
    }

    pub trait IndexStorage {
        fn store_index(&mut self, index: &Index) -> Result<()>;
        fn get_index(&self, id: &Uuid) -> Result<Option<Index>>;
        fn list_indices(&self) -> Result<Vec<Index>>;
    }
}
