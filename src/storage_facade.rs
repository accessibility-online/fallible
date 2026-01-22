// Contains abstractions for the calling layer to interface with any supported storage backend
// More to follow ...

use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Identifies the data store by backend type and ID / Location
/// 
/// Pattern matching prevents the need for checking string formats before accidentally parsing the wrong type
/// All types that are stored in this enum should be able to be stored and read by tools from the standard library or prelude
/// So parts of the program who haven't a clue what s3 is should be able to send it to methods that do.
pub enum DataStoreId {
    S3(String),
    Local(PathBuf),
}

/// Common metadata for any storage backend
/// 
/// It's not mandatory to use this in your backend implementations, though you'll be bullied if you don't :D
/// Metadata should be a public field which higher level layers can use to identify and distinguish different storage backends
pub struct StoreMetadata {
    pub id: DataStoreId,
    pub name: String,
    pub description: Option<String>,
}

/// Required trait for modules used to read and write directly to long term storage
pub trait StorageFacade {
    /// Reads binary data from a file at a path, optionally takes a decryption function.
    async fn read_data<F>(&self, path: &str, decrypt: Option<F>) -> Result<Vec<u8>, Box<dyn Error>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>>;

        /// Writes binary data to a file at a path, optionally takes an encryption function.
    async fn write_data<F>(&self, path: &str, data: &[u8], encrypt: Option<F>) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>>;

    /// Lists files at a given directory path
    async fn list_objects(&self, dir_path: &str) -> Result<Vec<String>, Box<dyn Error>>;

    /// Lists versions of a file at a filepath, originally intended for buckets but custom filesystem implementations are welcome
    async fn list_object_versions(&self, file_path: &str) -> Result<Vec<String>, Box<dyn Error>>;

    /// Deletes a file at a filepath
    async fn delete_file(&self, path: &str) -> Result<(), Box<dyn Error>>;

    /// Moves a file from one location to another, both paths must include the filename to facilitate renaming
    async fn move_file(&self, from: &str, to: &str) -> Result<(), Box<dyn Error>>;

    /// Copies a file from one location to another, both paths must include the filename to facilitate rename on copy
    async fn copy_file(&self, from: &str, to: &str) -> Result<(), Box<dyn Error>>;

    /// Returns a standard metadata object for a file at a given path
    async fn get_file_metadata(&self, path: &str) -> Result<fs::Metadata, Box<dyn Error>>;

    /// Checks if a file exists at a given path, cannot be used for directories
    async fn file_exists(&self, path: &str) -> bool;
}
