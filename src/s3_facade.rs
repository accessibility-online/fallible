use crate::storage_facade::{self, StorageFacade};
use aws_sdk_s3 as s3;
use aws_config;

pub struct S3Facade;

impl StorageFacade for  S3Facade {
    fn read_data<F>(&self, path: &str, decrypt: Option<F>) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn write_data<F>(&self, path: &str, data: &[u8], encrypt: Option<F>) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn list_objects(&self, dir_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn list_object_versions(&self, file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn delete_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn move_file(&self, from: &str, to: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn copy_file(&self, from: &str, to: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    
    fn get_file_metadata(&self, path: &str) -> Result<std::fs::Metadata, Box<dyn std::error::Error>> {
        todo!()
    }
    
    fn file_exists(&self, path: &str) -> bool {
        todo!()
    }
}