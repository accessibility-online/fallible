// Contains abstractions for the calling layer to interface with any supported storage backend
// More to follow ...

use std::error::Error;
use std::fs;
use std::future::Future;
use std::path::PathBuf;

/// Identifies the data store by backend type and ID / Location
///
/// Each case corresponds to a supported backend type, and the associated value is a system specific ID for a datastore on that backend.
/// This helps the calling layer know if it's dealing with an ARN, a local filesystem path or an Azure Blob URL, without implementing any logic beyond pattern matching the case.
/// All types that are stored in this enum should be able to be stored and read by tools from the standard library or prelude
/// So parts of the program who haven't a clue what s3 is should be able to send it to methods that do, saving everyone a headache and halving the coffee budget.
pub enum DataStoreId {
    S3(String),
    Local(PathBuf),
}

/// Common metadata for any storage backend
///
/// Note, cargo will bully you if you don't include one of these in your structs.
/// Higher level libraries should be able to use this data to distinguish between different storage backends instantiated in code, without worrying about how to perform platform specific operations.
/// think "Put this in the s3" or "Move this file from S3 to Wasabi"
/// This type is returned by the `metadata(&self) -> &storeMetadata` function, so callers can even use this on collections stored on the heap
/// 
/// # Parameters:
/// * id: Platform specific ID, EG ARN, Azure Blob storage url or B2 ID. In cases of a local FS, this should be a filepath to the root directory of the data store.
/// * name: Name of the data store. In the case of bucket storage, the name of the bucket. In the case of local fs facades, this should be the name of the data store directory.
/// *  description: What is this store for, or why does it need to exist. We've elected to make this mandatory for better oversight and auditability.
pub struct StoreMetadata {
    pub id: DataStoreId,
    pub name: String,
    pub description: String,
}

/// Required trait for modules used to read and write directly to long term storage
pub trait StorageFacade {
    /// Reads binary data from a file at a path, optionally takes a decryption function.
    fn read_data<F>(
        &self,
        path: &str,
        decrypt: Option<F>,
    ) -> impl Future<Output = Result<Vec<u8>, Box<dyn Error + Send + Sync>>> + Send
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> + Send + Sync;

    /// Writes binary data to a file at a path, optionally takes an encryption function.
    fn write_data<F>(
        &self,
        path: &str,
        data: &[u8],
        encrypt: Option<F>,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> + Send + Sync;

    /// Lists files at a given directory path
    /// 
    fn list_objects(
        &self,
        dir_path: &str,
    ) -> impl Future<Output = Result<Vec<String>, Box<dyn Error + Send + Sync>>> + Send;

    /// Lists versions of a file at a filepath, originally intended for buckets but custom filesystem implementations are welcome
    fn list_object_versions(
        &self,
        file_path: &str,
    ) -> impl Future<Output = Result<Vec<String>, Box<dyn Error + Send + Sync>>> + Send;

    /// Deletes a file at a filepath
    fn delete_file(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send;

    /// Moves a file from one location to another, both paths must include the filename to facilitate renaming
    fn move_file(
        &self,
        from: &str,
        to: &str,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send;

    /// Copies a file from one location to another, both paths must include the filename to facilitate rename on copy
    fn copy_file(
        &self,
        from: &str,
        to: &str,
    ) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send;

    /// Returns a standard metadata object for a file at a given path
    fn get_file_metadata(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<fs::Metadata, Box<dyn Error + Send + Sync>>> + Send;

    /// Checks if a file exists at a given path, cannot be used for directories
    fn file_exists(&self, path: &str) -> impl Future<Output = bool> + Send;

    /// Returns a reference to the metadata field of the struct
    fn metadata(&self) -> &StoreMetadata;
}
