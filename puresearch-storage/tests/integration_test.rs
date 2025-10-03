use puresearch_storage::MmapStorage;
use puresearch_core::{storage::StorageEngine, ReviewDocument};
use tempfile::tempdir;
use std::collections::HashMap;

#[test]
fn test_basic_storage_operations() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();
    
    let mut metadata = HashMap::new();
    metadata.insert("title".to_string(), "Test Review".to_string());
    metadata.insert("rating".to_string(), "5".to_string());
    
    let doc = ReviewDocument::new("This is a test review content".to_string(), metadata);
    let doc_id = doc.id;
    
    storage.store_document(&doc).unwrap();
    
    let retrieved_doc = storage.get_document(&doc_id).unwrap();
    assert!(retrieved_doc.is_some());
    let retrieved = retrieved_doc.unwrap();
    assert_eq!(retrieved.metadata.get("title").unwrap(), "Test Review");
    assert_eq!(retrieved.content, "This is a test review content");
    
    let doc_list = storage.list_documents().unwrap();
    assert_eq!(doc_list.len(), 1);
    assert!(doc_list.contains(&doc_id));
}

#[test]
fn test_persistence_and_recovery() {
    let temp_dir = tempdir().unwrap();
    
    let mut metadata = HashMap::new();
    metadata.insert("title".to_string(), "Persistent Test".to_string());
    
    let doc = ReviewDocument::new("This should survive restart".to_string(), metadata);
    let doc_id = doc.id;
    
    {
        let mut storage = MmapStorage::new(temp_dir.path()).unwrap();
        storage.store_document(&doc).unwrap();
        storage.flush().unwrap();
    }
    
    {
        let storage = MmapStorage::new(temp_dir.path()).unwrap();
        let retrieved_doc = storage.get_document(&doc_id).unwrap();
        assert!(retrieved_doc.is_some());
        let retrieved = retrieved_doc.unwrap();
        assert_eq!(retrieved.metadata.get("title").unwrap(), "Persistent Test");
        assert_eq!(retrieved.content, "This should survive restart");
    }
}

#[test] 
fn test_delete_operations() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();
    
    let mut metadata = HashMap::new();
    metadata.insert("title".to_string(), "To Delete".to_string());
    
    let doc = ReviewDocument::new("This will be deleted".to_string(), metadata);
    let doc_id = doc.id;
    
    storage.store_document(&doc).unwrap();
    assert!(storage.get_document(&doc_id).unwrap().is_some());
    
    let deleted = storage.delete_document(&doc_id).unwrap();
    assert!(deleted);
    assert!(storage.get_document(&doc_id).unwrap().is_none());
    
    let doc_list = storage.list_documents().unwrap();
    assert_eq!(doc_list.len(), 0);
}