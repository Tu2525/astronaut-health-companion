use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

pub(super) struct Log<T> {
    pub entries: Vec<T>,
    file: File,
    write_failed: bool,
}

impl<T: DeserializeOwned + Serialize> Log<T> {
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        let mut reader = BufReader::new(&file);
        let mut entries = Vec::new();
        let mut offset = 0;
        let mut line = Vec::new();
        while reader.read_until(b'\n', &mut line)? != 0 {
            if !line.ends_with(b"\n") {
                // An interrupted append can leave one unacknowledged partial line.
                file.set_len(offset)?;
                file.sync_data()?;
                break;
            }
            let entry = serde_json::from_slice(&line).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{} line {}: {error}", path.display(), entries.len() + 1),
                )
            })?;
            entries.push(entry);
            offset += line.len() as u64;
            line.clear();
        }
        Ok(Self {
            entries,
            file,
            write_failed: false,
        })
    }

    pub fn append(&mut self, entry: T) -> io::Result<()> {
        if self.write_failed {
            return Err(io::Error::other("message log requires recovery"));
        }
        let mut line = serde_json::to_vec(&entry)?;
        line.push(b'\n');
        let offset = self.file.seek(SeekFrom::End(0))?;
        if let Err(error) = self
            .file
            .write_all(&line)
            .and_then(|()| self.file.sync_data())
        {
            self.write_failed = self
                .file
                .set_len(offset)
                .and_then(|()| self.file.sync_data())
                .is_err();
            return Err(error);
        }
        self.entries.push(entry);
        Ok(())
    }
}
