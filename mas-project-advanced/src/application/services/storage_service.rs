use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait IStorageService: Send + Sync {
    async fn upload_file(&self, file_data: Vec<u8>, file_name: &str, bucket: &str) -> Result<String>;
    async fn delete_file(&self, file_url: &str) -> Result<()>;
}
