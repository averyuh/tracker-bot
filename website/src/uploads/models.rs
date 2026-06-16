use std::path::Path;

use nanoid::nanoid;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FileBlob {
    pub id: Uuid,
    pub file_path: String,
    pub hash: [u8; 32],
    pub size: i64,
    /* TODO: Store file format
        make sure file format matches file extension of File.
        File format will be detected by magic string.
    */ 
}

// TODO: Remove default
#[derive(Debug, Clone)]
pub struct File {
    pub id: Uuid,
    pub upload_id: String, // Sharable id
    pub blob_id: Uuid,
    pub original_name: String,
    pub created_at: OffsetDateTime,
}

impl File {
    pub fn new(original_name: String, blob_id: Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            upload_id: nanoid!(8), 
            blob_id,
            original_name,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.original_name)
            .extension()
            .and_then(|e| e.to_str())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileResponse {
    pub id: Uuid,
    pub file_name: String,
}

impl From<File> for FileResponse {
    fn from(file: File) -> Self {
        Self {
            id: file.id,
            file_name: file.original_name,
        }
    }
}
