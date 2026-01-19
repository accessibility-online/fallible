use std::error::Error;

pub trait StorageFacade {
    fn read_data<F>(&self, path: &str, decrypt: Option<F>) -> Result<Vec<u8>, Box<dyn Error>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>>;

    fn write_data<F>(&self, path: &str, data: &[u8], encrypt: Option<F>) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>>;

    fn list_objects(&self, dir_path: &str) -> Result<Vec<String>, Box<dyn Error>>;

    fn list_object_versions(&self, file_path: &str) -> Result<Vec<String>, Box<dyn Error>>;

    fn delete_file(&self, path: &str) -> Result<(), Box<dyn Error>>;

    fn move_file(&self, from: &str, to: &str) -> Result<(), Box<dyn Error>>;

    fn copy_file(&self, from: &str, to: &str) -> Result<(), Box<dyn Error>>;
}
