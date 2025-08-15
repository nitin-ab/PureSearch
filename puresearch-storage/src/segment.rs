use anyhow::Result;
use memmap2::{Mmap, MmapMut};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, Seek, SeekFrom};
use std::path::Path;

pub struct SegmentFile {
    file: File,
    mmap: Option<Mmap>,
    size: usize,
}

impl SegmentFile {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;
        
        Ok(Self {
            file,
            mmap: None,
            size: 0,
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;
        
        let metadata = file.metadata()?;
        let size = metadata.len() as usize;
        
        let mmap = if size > 0 {
            Some(unsafe { Mmap::map(&file)? })
        } else {
            None
        };

        Ok(Self {
            file,
            mmap,
            size,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        let mut writer = BufWriter::new(&mut self.file);
        writer.seek(SeekFrom::End(0))?;
        let offset = writer.stream_position()? as usize;
        writer.write_all(data)?;
        writer.flush()?;
        
        self.size += data.len();
        self.mmap = None;
        
        Ok(offset)
    }

    pub fn read_at(&mut self, offset: usize, len: usize) -> Result<Vec<u8>> {
        if self.mmap.is_none() && self.size > 0 {
            self.mmap = Some(unsafe { Mmap::map(&self.file)? });
        }

        if let Some(mmap) = &self.mmap {
            if offset + len <= mmap.len() {
                Ok(mmap[offset..offset + len].to_vec())
            } else {
                Err(anyhow::anyhow!("Read beyond segment bounds"))
            }
        } else {
            Err(anyhow::anyhow!("Cannot read from empty segment"))
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}