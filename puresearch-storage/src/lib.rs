use anyhow::Result;
use memmap2::{Mmap, MmapMut};
use puresearch_core::{storage::{StorageEngine, IndexStorage}, ReviewDocument, Index};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub mod segment;
pub mod wal;

pub use segment::SegmentFile;
pub use wal::WriteAheadLog;

pub struct MmapStorage {
    data_dir: PathBuf,
    documents: HashMap<Uuid, ReviewDocument>,
    indices: HashMap<Uuid, Index>,
    wal: WriteAheadLog,
}

impl MmapStorage {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&data_dir)?;
        
        let wal = WriteAheadLog::new(data_dir.join("wal.log"))?;
        
        Ok(Self {
            data_dir,
            documents: HashMap::new(),
            indices: HashMap::new(),
            wal,
        })
    }
}

impl StorageEngine for MmapStorage {
    fn store_document(&mut self, doc: &ReviewDocument) -> Result<()> {
        self.wal.write_document_entry(doc)?;
        self.documents.insert(doc.id, doc.clone());
        Ok(())
    }

    fn get_document(&self, id: &Uuid) -> Result<Option<ReviewDocument>> {
        Ok(self.documents.get(id).cloned())
    }

    fn delete_document(&mut self, id: &Uuid) -> Result<bool> {
        self.wal.write_delete_entry(id)?;
        Ok(self.documents.remove(id).is_some())
    }

    fn list_documents(&self) -> Result<Vec<Uuid>> {
        Ok(self.documents.keys().copied().collect())
    }
}

impl IndexStorage for MmapStorage {
    fn store_index(&mut self, index: &Index) -> Result<()> {
        self.indices.insert(index.id, index.clone());
        Ok(())
    }

    fn get_index(&self, id: &Uuid) -> Result<Option<Index>> {
        Ok(self.indices.get(id).cloned())
    }

    fn list_indices(&self) -> Result<Vec<Index>> {
        Ok(self.indices.values().cloned().collect())
    }
}
