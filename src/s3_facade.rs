// Provides abstractions for calling layers to interface with AWS s3
// 
// More to follow
use crate::storage_facade::StoreMetadata;
use aws_sdk_s3 as s3;
use aws_config as config;

/// Contains the client and metadata as fields
pub struct s3Facade {
    client: s3::Client,
    metadata: StoreMetadata,
}