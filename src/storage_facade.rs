use std::error::Error;
use std::fs;

/// Required trait for modules used to read and write directly to long term storage
/// 
/// This trait outlines the common headers to be shared between different storage facade modules.
/// The general idea being that if you use any of these methods, and avoid using module specific methods where possible, migrating a storage service will be as simple as switching module names.
/// 
/// There are Some areas in which facades will differ, for example buckets do not require directory creation, and local filesystems do. 
/// In cases like this, methods such as `move_file(&self, from: &str, to: &str) -> Result<(), Box<dyn Error>>`  should be the public method, with calls to private functions to check directories exist, and create them if not.
/// 
/// All functions defined here should return a Result<> object with a type or () as the OK value, and a generic error type using Box as the Err value.
/// This is to reduce SDK dependence for proprietary code, and to reduce complexity for implementation of business logic.
/// 
/// Likewise, where OK values are defined, Standard Library return types are prefered, unless absolutely necessary.
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
