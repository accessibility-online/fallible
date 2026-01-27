// Provides abstractions for managing data in S3 Buckets
//
// This module provides abstractions for working with data in S3 buckets directly.
// It conforms to the [`StorageFacade`] trait, and use of the trait methods above any module specific implementations is heavily encouraged, see below.
// 
// **NOTE:** This module assumes you have preexisting AWS assets, either via IAC or through manual creation.
// While facades working on more traditional filesystems may be able to create their silos automatically, we liken the creation or destruction of a bucket in a similar light to creating or destroying a server.
// Plus, there are additional considerations around bucket creation, such as public access blocks, policies around IAM access, and other details far beyond the scope of a module designed to abstract the storage and retrieval of data.
// 
// Where this module takes advantage of S3 specific features, it either does so within the bodies of public methods, or abstracts calls into private methods.
// The idea being that if we use an S3 specific feature, it should be part of a process that can be considered agnostic to all structs which implement the StorageFacade trait.
// for example, methods checking storage class of a file, and potentially triggering a move from deep archive to instant access, should be called as part of a process within a public method.
// This way, callers don't need to care about or work with the platform specific features of each data store, but can implement high level instructions which will take advantage of them if required.
use crate::storage_facade::{DataStoreId, StorageFacade, StoreMetadata};
use aws_config as aws;
use aws_sdk_s3::{self as s3, error::SdkError, operation::{head_object::{HeadObjectError, HeadObjectOutput}, list_objects_v2::{ListObjectsV2Error, ListObjectsV2Output}}, primitives::ByteStream};
use std::error::Error;

/// Contains the client and metadata as fields
pub struct S3Facade {
    client: s3::Client,
    metadata: StoreMetadata,
}

impl S3Facade {
    /// Constructor with bucket exists logic
    ///
    /// This constructor returns the S3Facade struct if the name argument matches the name of an existing s3 bucket.
    /// We use head_bucket() as a lightweight call to check for bucket existence.
    /// If the bucket exists, we return an S3Facade struct containing metadata useful to the calling layer. If not, we return an error, and will log specifics from here.
    /// In all cases, we want to return an arn for the bucket as part of the metadata, even if one is not provided by the sdk. This is because AWS is known to use bucket names and arns for different purposes, and we want to cover all bases.
    /// If the SDK is unable to return the ARN automatically, we construct it using String::format();
    pub async fn new(name: &str, description: &str) -> Result<Self, Box<dyn Error>> {
        let config = aws::load_defaults(aws::BehaviorVersion::v2026_01_12()).await;
        let client = s3::Client::new(&config);

        let request = client.head_bucket().bucket(name).send().await;

        match request {
            Err(e) => {
                // Logging logic goes here
                return Err(e.into());
            }
            Ok(result) => {
                let arn = result
                    .bucket_arn()
                    .map(String::from)
                    .unwrap_or_else(|| format!("arn:aws:s3:::{}", name));

                let facade = S3Facade {
                    client,
                    metadata: StoreMetadata {
                        id: DataStoreId::S3(arn),
                        name: name.to_string(),
                        description: description.to_string()
                    },
                };

                Ok(facade)
            }
        }
    }

    async fn get_object_head(&self, path: &str) -> Result<HeadObjectOutput, SdkError<HeadObjectError>> {
let head = self.client.head_object()
        .bucket(&self.metadata.name)
        .key(path)
        .send()
        .await?;

Ok(head)
    }
}

impl StorageFacade for S3Facade {
    /// Reads binary data from a file in an S3 Bucket
    /// 
    /// # Remarks
    /// Designed to read files from an s3 bucket and return raw binary data. This can be used on larger files, though it will block the thread until the file is read.
    /// We intend to implement streaming functions for larger files measured in GBs and TBs, after which a size limit will be imposed on the use of this function.
    /// 
    /// # Arguments
    /// * `path` - the path of the file to read, using forward slash "/" separators
    /// * `decrypt` - An optional function which can be parsed in to decrypt raw bytes before they are returned to the calling layer
    /// 
    /// # Examples
    async fn read_data<F>(
        &self,
        path: &str,
        decrypt: Option<F>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
    {
// When ready, call get_file_metadata here to check size before reading

        let data = self
            .client
            .get_object()
            .bucket(&self.metadata.name)
            .key(path)
            .send()
            .await?;

        let bytes = data.body.collect().await?.into_bytes();

        if let Some(decrypt_fn) = decrypt {
            let cleartext = decrypt_fn(&bytes);
            match cleartext {
                Ok(bytes) => return Ok(bytes),
                Err(e) => return Err(e.into())
            }
        };

        Ok(Vec::from(bytes))
    }

    /// Writes a byte-slice to an S3 bucket and returns result
    /// 
    /// This function does not take ownership, allowing callers to continue using data due to be written, if required.
    /// The tradeoff is that this function adopts the slight overhead of copying referenced data into a vector owned by the function.
    /// We do this as part of the encrypt operation if an encryption function has been parsed, and as part of the else if one has not.
    /// As with the read_data function, this operation blocks a thread until the file write is complete. Whilst it works with large uploads, we intend to write a streaming or multi-part upload function for files measured in GBs and TBs.
    async fn write_data<F>(
        &self,
        path: &str,
        data: &[u8],
        encrypt: Option<F>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
    {
        let data = if let Some(encrypt_fn) = encrypt {
encrypt_fn(data)?
        } else {
            data.to_vec()
        };

        let upload = self
            .client
            .put_object()
            .bucket(&self.metadata.name)
            .key(path)
            .body(ByteStream::from(data))
            .send()
            .await;

        if let Err(e) = upload {
            // ToDo put some error logging code here with tracing
            return Err(e.into());
        }

        Ok(())
    }

    /// Lists objects with a given prefix in an S3 bucket, returned in lexicographical alphabetical order
    /// 
    /// Callers note that due to the nature of bucket storage, flat structure means this function will list all objects in all contained directories within the specified directory
    /// For speed, we are electing to keep this as is for now, so you may need to filter your output lists.
    /// either that, or it will save you a few extra cpu cycles for recursive listings down the tree.
    async fn list_objects(
        &self,
        dir_path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let request = self.client.list_objects_v2()
            .bucket(&self.metadata.name)
            .prefix(dir_path)
            .into_paginator()
            .send();

            // Takes each optional pagination object, and unwraps it into a vector of pagination objects.
        let pages: Vec<ListObjectsV2Output> = request
            .collect::<Result<Vec<ListObjectsV2Output>, SdkError<ListObjectsV2Error>>>()
            .await?;

        let mut keys: Vec<String> = Vec::new();
        for p in pages {
            for object in p.contents() {
                if let Some(key) = object.key() {
                    keys.push(key.to_string());
                }
            }
        }
        keys.sort();
        Ok(keys)
    }

    async fn list_object_versions(
        &self,
        file_path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let request = self.client.list_object_versions()
        .bucket(&self.metadata.name)
        .prefix(file_path)
        .send()
        .await?;

    let mut versions = request.versions.unwrap();
    let mut next_key_marker = request.next_key_marker.unwrap_or_default();
    let mut next_version_id_marker = request.next_version_id_marker.unwrap_or_default();
    let mut truncated = request.is_truncated.unwrap_or(false);
    while truncated {
        let next_request = self.client.list_object_versions()
        .bucket(&self.metadata.name)
        .prefix(file_path)
        .key_marker(&next_key_marker)
        .version_id_marker(&next_version_id_marker)
        .send()
        .await?;

        versions.extend_from_slice(next_request.versions());
        next_key_marker = next_request.next_key_marker.unwrap_or_default();
        next_version_id_marker = next_request.next_version_id_marker.unwrap_or_default();

    truncated = next_request.is_truncated.unwrap_or(false);
    }

        todo!()
    }

    async fn delete_file(
        &self,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _request = self.client.delete_object()
        .bucket(&self.metadata.name)
        .key(path)
        .send()
        .await?;

    Ok(())
    }

    async fn move_file(
        &self,
        from: &str,
        to: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
self.copy_file(from, to).await?;
        self.delete_file(from).await?;

        Ok(())
    }

    /// Copies a file from one location to another within the same bucket
    /// 
    /// The design choice was taken to keep copy operations within the same bucket, due to the nature of how the AWS SDK expects to work with the copy_source string.
    /// We use the bucket name stored in the struct's metadata prepended to the copy source to fulfill this requirement.
    /// Eventually, we should look at creating a migrate function which is able to not only work between two S3 buckets, but also be completely backend agnostic, using server side operations when possible and performing carefully managed copies between source and dest when not.
    async fn copy_file(
        &self,
        from: &str,
        to: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _request = self.client.copy_object()
        .copy_source(format!("{}/{}", &self.metadata.name, from))
        .bucket(&self.metadata.name)
.key(to)
.send()
.await?;

Ok(())
    }

    async fn file_exists(&self, path: &str) -> bool {
let check = self.get_object_head(path).await;

    if let Ok(_) = check {
        true
    } else {
        false
    }
    }

    fn metadata(&self) -> &StoreMetadata {
        &self.metadata
    }
}
