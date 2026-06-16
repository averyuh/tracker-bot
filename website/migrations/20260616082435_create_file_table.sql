CREATE TABLE file_blobs (
    id UUID PRIMARY KEY,
    storage_path TEXT NOT NULL,

    hash BYTEA NOT NULL,
    size BIGINT NOT NULL,

    file_type TEXT
);

-- dedup lookup
CREATE UNIQUE INDEX idx_file_blobs_hash
ON file_blobs(hash);

CREATE TABLE files (
    id UUID PRIMARY KEY,

    upload_id TEXT NOT NULL UNIQUE,

    blob_id UUID NOT NULL REFERENCES file_blobs(id) ON DELETE RESTRICT,

    original_name TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_files_blob_id
ON files(blob_id);

CREATE INDEX idx_files_upload_id
ON files(upload_id);