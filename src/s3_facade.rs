// Provides abstractions for calling layers to interface with AWS s3
//
// More to follow
use std::error::Error;
use crate::storage_facade::{DataStoreId, StorageFacade, StoreMetadata};
use aws_config as aws;
use aws_sdk_s3::{self as s3};

/// Contains the client and metadata as fields
pub struct S3Facade {
    client: s3::Client,
    metadata: StoreMetadata,
}

impl S3Facade {
    /// Constructor with bucket exists logic
    /// 
    /// This constructor returns the S3Facade struct if the name argument matches the name of an existing s3 bucket.
    /// the head_bucket method provides a lightweight request returning bucket information from the name, and is used as a standard way to check existance and authorisation.
    /// Handily, it also returns data which is used to populate the metadata fields of the struct, such as id.
    pub async fn new(name: &str, description: &str) -> Result<Self, Box<dyn Error>> {
        let config = aws::load_defaults(aws::BehaviorVersion::v2026_01_12()).await;
        let client = s3::Client::new(&config);

        let request = client.head_bucket().bucket(name).send().await;

        Ok(S3Facade {
            client,
            metadata: StoreMetadata {
                id: DataStoreId::S3("".to_string()),
                name: name.to_string(),
                description: Some(description.to_string()),
            },
        })
    }
}

impl StorageFacade for S3Facade {
    async fn read_data<F>(
        &self,
        path: &str,
        decrypt: Option<F>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
    {
        let data = self
            .client
            .get_object()
            .bucket(&self.metadata.name)
            .key(path)
            .send()
            .await?;

        let bytes = data.body.collect().await?.into_bytes();

        if let Some(decrypt_fn) = decrypt {
            println!("Encryption is currently not implemented, returning raw data.");
            return Ok(Vec::from(bytes));
        }

        Ok(Vec::from(bytes))
    }

    async fn write_data<F>(
        &self,
        path: &str,
        data: &[u8],
        encrypt: Option<F>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
    {
        todo!()
    }

    async fn list_objects(
        &self,
        dir_path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn list_object_versions(
        &self,
        file_path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn delete_file(
        &self,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn move_file(
        &self,
        from: &str,
        to: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn copy_file(
        &self,
        from: &str,
        to: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn get_file_metadata(
        &self,
        path: &str,
    ) -> Result<std::fs::Metadata, Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn file_exists(&self, path: &str) -> bool {
        todo!()
    }

    fn metadata(&self) -> &StoreMetadata {
        todo!()
    }
}
