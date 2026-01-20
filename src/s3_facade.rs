use crate::storage_facade::{self, StorageFacade};
use aws_sdk_s3 as s3;

pub struct S3Facade {
    client: s3::Client
}

impl S3Facade {
    pub async fn new(config: s3::config::Config) -> Self {
        let client = s3::Client::from_conf(config);
        
        S3Facade { 
            client 
        }
    }
}

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