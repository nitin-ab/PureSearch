use anyhow::Result;
use puresearch_core::ReviewDocument;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum WalEntry {
    Document(ReviewDocument),
    Delete(Uuid),
}

pub struct WriteAheadLog {
    writer: BufWriter<File>,
}

impl WriteAheadLog {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub fn write_document_entry(&mut self, doc: &ReviewDocument) -> Result<()> {
        let entry = WalEntry::Document(doc.clone());
        self.write_entry(&entry)
    }

    pub fn write_delete_entry(&mut self, id: &Uuid) -> Result<()> {
        let entry = WalEntry::Delete(*id);
        self.write_entry(&entry)
    }

    fn write_entry(&mut self, entry: &WalEntry) -> Result<()> {
        let serialized = bincode::serialize(entry)?;
        let len = serialized.len() as u32;
        
        self.writer.write_all(&len.to_le_bytes())?;
        self.writer.write_all(&serialized)?;
        self.writer.flush()?;
        
        Ok(())
    }

    pub fn sync(&mut self) -> Result<()> {
        self.writer.flush()?;
        self.writer.get_ref().sync_all()?;
        Ok(())
    }
}