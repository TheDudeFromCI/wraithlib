use rusqlite::{Connection, DatabaseName, Result};

use crate::common::uuid::Uuid;

pub trait BlobIO {
    /// Read a blob from the database.
    fn read_blob_exact(&self, table: &str, col: &str, row: i64, bytes: usize) -> Result<Vec<u8>>;

    /// Read a blob from the database.
    fn read_blob(&self, table: &str, col: &str, row: i64) -> Result<Vec<u8>>;

    /// Write a blob to the database.
    fn write_blob(&self, table: &str, col: &str, row: i64, data: &[u8]) -> Result<()>;

    /// Read a 16-byte UUID from the database.
    fn read_uuid(&self, table: &str, col: &str, row: i64) -> Result<Uuid> {
        let bytes = self.read_blob_exact(table, col, row, 16)?;
        let Ok(bytes) = bytes.try_into() else {
            return Err(rusqlite::Error::BlobSizeError);
        };
        Ok(Uuid::from_bytes(bytes))
    }

    /// Write a 16-byte UUID to the database.
    fn write_uuid(&self, table: &str, col: &str, row: i64, uuid: &Uuid) -> Result<()> {
        self.write_blob(table, col, row, uuid.as_bytes())
    }
}

impl BlobIO for Connection {
    fn read_blob_exact(&self, table: &str, col: &str, row: i64, bytes: usize) -> Result<Vec<u8>> {
        let mut data = vec![0u8; bytes];
        let blob = self.blob_open(DatabaseName::Main, table, col, row, true)?;
        blob.read_at_exact(&mut data, 0)?;
        Ok(data)
    }

    fn read_blob(&self, table: &str, col: &str, row: i64) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut data = [0u8; 1024];

        let blob = self.blob_open(DatabaseName::Main, table, col, row, true)?;

        let mut offset = 0;
        loop {
            let read = blob.read_at(&mut data, offset)?;
            if read == 0 {
                break;
            }

            bytes.extend_from_slice(&data[.. read]);
            offset += read;
        }

        Ok(bytes)
    }

    fn write_blob(&self, table: &str, col: &str, row: i64, data: &[u8]) -> Result<()> {
        self.blob_open(DatabaseName::Main, table, col, row, false)?
            .write_at(data, 0)?;

        Ok(())
    }
}
