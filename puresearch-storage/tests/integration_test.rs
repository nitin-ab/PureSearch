use puresearch_storage::MmapStorage;
use puresearch_core::{storage::StorageEngine, ReviewDocument};
use tempfile::tempdir;
use std::collections::HashMap;

// Add imports
use puresearch_core::{Index, storage::IndexStorage};
use uuid::Uuid;

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

#[test]
fn test_basic_index_operations() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();
    
    let mut index = Index::new("test_index".to_string());
    let index_id = index.id;
    
    storage.store_index(&index).unwrap();
    
    let retrieved = storage.get_index(&index_id).unwrap().unwrap();
    assert_eq!(retrieved.name, "test_index");
    assert!(retrieved.documents.is_empty());
    
    let mut metadata = HashMap::new();
    let doc = ReviewDocument::new("Test content".to_string(), metadata);
    let doc_id = doc.id;
    storage.store_document(&doc).unwrap();
    
    index.add_document(doc_id);
    storage.store_index(&index).unwrap();
    
    let retrieved = storage.get_index(&index_id).unwrap().unwrap();
    assert_eq!(retrieved.documents.len(), 1);
    assert!(retrieved.documents.contains(&doc_id));
    
    let indices = storage.list_indices().unwrap();
    assert_eq!(indices.len(), 1);
}

#[test]
fn test_index_persistence() {
    let temp_dir = tempdir().unwrap();
    
    let index_id;
    let doc_id;
    {
        let mut storage = MmapStorage::new(temp_dir.path()).unwrap();
        
        let mut index = Index::new("persistent_index".to_string());
        index_id = index.id;
        
        let mut metadata = HashMap::new();
        let doc = ReviewDocument::new("Persistent content".to_string(), metadata);
        doc_id = doc.id;
        storage.store_document(&doc).unwrap();
        
        index.add_document(doc_id);
        storage.store_index(&index).unwrap();
        storage.flush().unwrap();
    }
    
    {
        let storage = MmapStorage::new(temp_dir.path()).unwrap();
        let retrieved = storage.get_index(&index_id).unwrap().unwrap();
        assert_eq!(retrieved.name, "persistent_index");
        assert_eq!(retrieved.documents.len(), 1);
        assert!(retrieved.documents.contains(&doc_id));
    }
}

#[test]
fn test_multiple_indices() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();

    let index1 = Index::new("index1".to_string());
    let index2 = Index::new("index2".to_string());

    storage.store_index(&index1).unwrap();
    storage.store_index(&index2).unwrap();

    let indices = storage.list_indices().unwrap();
    assert_eq!(indices.len(), 2);

    let retrieved1 = storage.get_index(&index1.id).unwrap().unwrap();
    assert_eq!(retrieved1.name, "index1");

    let retrieved2 = storage.get_index(&index2.id).unwrap().unwrap();
    assert_eq!(retrieved2.name, "index2");
}

#[test]
fn test_document_update() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();

    let mut metadata = HashMap::new();
    metadata.insert("title".to_string(), "Original".to_string());

    let original_doc = ReviewDocument {
        id: Uuid::new_v4(),
        content: "Original content".to_string(),
        metadata: metadata.clone(),
        timestamp: 0,
    };
    storage.store_document(&original_doc).unwrap();

    let mut updated_doc = original_doc.clone();
    updated_doc.content = "Updated content".to_string();
    updated_doc.metadata.insert("title".to_string(), "Updated".to_string());

    storage.store_document(&updated_doc).unwrap();

    let retrieved = storage.get_document(&original_doc.id).unwrap().unwrap();
    assert_eq!(retrieved.content, "Updated content");
    assert_eq!(retrieved.metadata.get("title").unwrap(), "Updated");
}

#[test]
fn test_non_existent_operations() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();

    let fake_id = Uuid::new_v4();

    let doc = storage.get_document(&fake_id).unwrap();
    assert!(doc.is_none());

    let deleted = storage.delete_document(&fake_id).unwrap();
    assert!(!deleted);

    let index = storage.get_index(&fake_id).unwrap();
    assert!(index.is_none());
}

#[test]
fn test_empty_document() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();

    let empty_doc = ReviewDocument::new("".to_string(), HashMap::new());
    let doc_id = empty_doc.id;

    storage.store_document(&empty_doc).unwrap();

    let retrieved = storage.get_document(&doc_id).unwrap().unwrap();
    assert_eq!(retrieved.content, "");
    assert!(retrieved.metadata.is_empty());
}

#[test]
fn test_index_with_duplicates() {
    let temp_dir = tempdir().unwrap();
    let mut storage = MmapStorage::new(temp_dir.path()).unwrap();

    let mut index = Index::new("dup_test".to_string());

    let doc = ReviewDocument::new("Test".to_string(), HashMap::new());
    let doc_id = doc.id;
    storage.store_document(&doc).unwrap();

    index.add_document(doc_id);
    index.add_document(doc_id); // duplicate add

    storage.store_index(&index).unwrap();

    let retrieved = storage.get_index(&index.id).unwrap().unwrap();
    assert_eq!(retrieved.documents.len(), 1);
    assert!(retrieved.documents.contains(&doc_id));
}