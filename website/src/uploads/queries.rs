use sqlx::{PgPool};
use uuid::Uuid;

use crate::uploads::models::{File, FileBlob, FileBlobRow};

pub async fn find_file(
    db: &PgPool,
    id: Uuid,
) -> Result<File, sqlx::Error> {
    let file = sqlx::query_as!(
        File,
        r#"
        SELECT *
        FROM files
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    Ok(file)
}



pub async fn find_blob(
    db: &PgPool,
    id: Uuid,
) -> anyhow::Result<FileBlob> {
    let file = sqlx::query_as!(
        FileBlobRow,
        r#"
        SELECT *
        FROM file_blobs
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    file.try_into()
}