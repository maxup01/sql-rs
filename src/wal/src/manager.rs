use crate::{WALEntry, WALError};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    path::{Path, PathBuf},
};

pub struct WALManager {
    root: PathBuf,
    files: HashMap<String, File>,
}

impl WALManager {
    pub fn open<P: AsRef<Path>>(root: P) -> crate::Result<Self> {
        let exists = std::fs::exists(&root)?;
        let path = root.as_ref().to_path_buf();

        if !exists {
            return Err(WALError::WALDirectoryNotFound(path));
        }

        let mut files = HashMap::new();

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            let file_name = entry.file_name();

            // Skip files that are not segments, e.g. a future LOCK or header file
            let Some(file_name) = file_name.to_str() else {
                continue;
            };

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(entry.path())?;
            let _ = file.seek(SeekFrom::End(0))?;

            files.insert(file_name.to_string(), file);
        }

        Ok(Self { root: path, files })
    }
}
