use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug)]
enum LogRecord {
    Set { key: String, value: String },
    Remove { key: String },
}

pub struct KvStore {
    file: File,
    index: HashMap<String, u64>,
}

impl KvStore {
    pub fn open(path: impl AsRef<Path>) -> Result<KvStore> {
        let path = path.as_ref();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .context("Failed to open database file")?;

        let mut index = HashMap::new();
        let mut pos = 0;

        loop {
            let start_pos = pos;
            match bincode::deserialize_from::<&File, LogRecord>(&file) {
                Ok(record) => {
                    match record {
                        LogRecord::Set { key, .. } => {
                            index.insert(key, start_pos);
                        }
                        LogRecord::Remove { key } => {
                            index.remove(&key);
                        }
                    }
                    pos = file.stream_position()?;
                }
                Err(_) => break, // EOF or error
            }
        }

        Ok(KvStore { file, index })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let record = LogRecord::Set { key: key.clone(), value };
        let pos = self.file.seek(SeekFrom::End(0))?;
        
        bincode::serialize_into(&mut self.file, &record)?;
        self.file.flush()?;
        
        self.index.insert(key, pos);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(&pos) = self.index.get(&key) {
            self.file.seek(SeekFrom::Start(pos))?;
            match bincode::deserialize_from(&mut self.file)? {
                LogRecord::Set { value, .. } => Ok(Some(value)),
                LogRecord::Remove { .. } => Ok(None), // Should not happen if index is correct
            }
        } else {
            Ok(None)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let record = LogRecord::Remove { key: key.clone() };
            bincode::serialize_into(&mut self.file, &record)?;
            self.file.flush()?;
            
            self.index.remove(&key);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Key not found"))
        }
    }
}
