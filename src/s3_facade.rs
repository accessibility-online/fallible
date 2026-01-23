// Provides abstractions for calling layers to interface with AWS s3
// 
// More to follow
use crate::storage_facade::{StorageFacade, StoreMetadata};
use aws_sdk_s3 as s3;

/// Contains the client and metadata as fields
pub struct S3Facade {
    client: s3::Client,
    metadata: StoreMetadata,
}

impl StorageFacade for S3Facade {
    async fn read_data<F>(&self, path: &str, decrypt: Option<F>) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync {
        todo!()
    }

    async fn write_data<F>(&self, path: &str, data: &[u8], encrypt: Option<F>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync {
        todo!()
    }

    async fn list_objects(&self, dir_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn list_object_versions(&self, file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn delete_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn move_file(&self, from: &str, to: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn copy_file(&self, from: &str, to: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn get_file_metadata(&self, path: &str) -> Result<std::fs::Metadata, Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }

    async fn file_exists(&self, path: &str) -> bool {
        todo!()
    }

    fn metadata(&self) -> &StoreMetadata {
        todo!()
    }
}