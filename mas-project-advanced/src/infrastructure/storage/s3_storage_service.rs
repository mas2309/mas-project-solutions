use crate::application::services::storage_service::IStorageService;
use crate::shared::config::StorageConfig;
use async_trait::async_trait;
use anyhow::{Result, Context};
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::config::{Credentials, Region, BehaviorVersion};

/// Servicio de almacenamiento compatible con S3 para Contabo Object Storage.
/// Soporta múltiples buckets y endpoint personalizado.
pub struct ContaboStorageService {
    client: Client,
    endpoint: String,
    tenant_id: String,
}

impl ContaboStorageService {
    pub async fn new(config: &StorageConfig) -> Result<Self> {
        let credentials = Credentials::new(
            &config.access_key,
            &config.secret_key,
            None,
            None,
            "contabo",
        );

        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(&config.endpoint)
            .region(Region::new(config.region.clone()))
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        Ok(Self {
            client,
            endpoint: config.endpoint.clone(),
            tenant_id: config.tenant_id.clone(),
        })
    }

    /// Construye la URL pública del archivo en Contabo
    /// Formato: https://{endpoint}/{tenant_id}:{bucket}/{key}
    fn build_url(&self, bucket: &str, key: &str) -> String {
        format!(
            "{}/{}:{}/{}",
            self.endpoint, self.tenant_id, bucket, key
        )
    }

    /// Extrae el bucket y key desde una URL de Contabo
    /// URL formato: https://usc1.contabostorage.com/{tenant_id}:{bucket}/{key}
    fn parse_url(&self, file_url: &str) -> Option<(String, String)> {
        // Buscar el patrón tenant_id:bucket/key
        let search = format!("{}:", self.tenant_id);
        if let Some(pos) = file_url.find(&search) {
            let remainder = &file_url[pos + search.len()..];
            if let Some(slash_pos) = remainder.find('/') {
                let bucket = &remainder[..slash_pos];
                let key = &remainder[slash_pos + 1..];
                return Some((bucket.to_string(), key.to_string()));
            }
        }
        None
    }
}

#[async_trait]
impl IStorageService for ContaboStorageService {
    async fn upload_file(&self, file_data: Vec<u8>, file_name: &str, bucket: &str) -> Result<String> {
        println!("📤 Subiendo '{}' al bucket: {} (Contabo)", file_name, bucket);

        let body = ByteStream::from(file_data);

        self.client
            .put_object()
            .bucket(bucket)
            .key(file_name)
            .body(body)
            .send()
            .await
            .context(format!("Error al subir archivo '{}' al bucket '{}'", file_name, bucket))?;

        let file_url = self.build_url(bucket, file_name);
        println!("✅ Archivo subido: {}", file_url);
        Ok(file_url)
    }

    async fn delete_file(&self, file_url: &str) -> Result<()> {
        println!("🗑️ Eliminando: {}", file_url);

        if let Some((bucket, key)) = self.parse_url(file_url) {
            self.client
                .delete_object()
                .bucket(&bucket)
                .key(&key)
                .send()
                .await
                .context(format!("Error al eliminar '{}' del bucket '{}'", key, bucket))?;

            println!("✅ Archivo eliminado");
        } else {
            println!("⚠️ No se pudo parsear la URL para eliminar: {}", file_url);
        }

        Ok(())
    }
}
