use std::error::Error;

pub trait StorageFacade {
    fn read_data(&self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn write_data(&mut self, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
    fn list_objects(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn list_object_versions(&self) -> Result<Vec<String>, Box<dyn Error>>;
}

