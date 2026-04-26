/// S3 storage client for file upload/download
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::{Credentials, Region};
use bytes::Bytes;

use crate::config::S3Settings;

/// Wrapper around the AWS S3 client
#[derive(Clone)]
pub struct StorageClient {
    client: S3Client,
    pub bucket_uploads: String,
    pub bucket_audio: String,
}

impl StorageClient {
    /// Create a new S3 client from settings
    pub async fn new(settings: &S3Settings) -> Self {
        let credentials = Credentials::new(
            &settings.access_key,
            &settings.secret_key,
            None,
            None,
            "env",
        );

        let config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(&settings.endpoint)
            .region(Region::new("us-east-1"))
            .credentials_provider(credentials)
            .force_path_style(true) // Required for MinIO
            .build();

        let client = S3Client::from_conf(config);

        Self {
            client,
            bucket_uploads: settings.bucket_uploads.clone(),
            bucket_audio: settings.bucket_audio.clone(),
        }
    }

    /// Ensure required buckets exist (create if missing)
    pub async fn ensure_buckets(&self) -> Result<(), Box<dyn std::error::Error>> {
        for bucket in [&self.bucket_uploads, &self.bucket_audio] {
            match self.client.head_bucket().bucket(bucket).send().await {
                Ok(_) => {}
                Err(_) => {
                    self.client
                        .create_bucket()
                        .bucket(bucket)
                        .send()
                        .await?;
                    tracing::info!("Created S3 bucket: {}", bucket);
                }
            }
        }
        Ok(())
    }

    /// Upload a file to the uploads bucket
    pub async fn upload_file(
        &self,
        key: &str,
        data: Bytes,
        content_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .put_object()
            .bucket(&self.bucket_uploads)
            .key(key)
            .body(data.into())
            .content_type(content_type)
            .send()
            .await?;
        Ok(())
    }

    /// Download a file from the uploads bucket
    pub async fn download_file(
        &self,
        key: &str,
    ) -> Result<Bytes, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket_uploads)
            .key(key)
            .send()
            .await?;
        let data = resp.body.collect().await?;
        Ok(data.into_bytes())
    }
}
