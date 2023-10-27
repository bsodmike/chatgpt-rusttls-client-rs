use thiserror::Error;

#[allow(dead_code)]
pub type Error = Box<dyn std::error::Error>;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum MyError {
    // Define your custom error types here
}
