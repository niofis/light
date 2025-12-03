mod file_loader;
mod zip_loader;
use anyhow::Result;
pub use file_loader::FileLoader;
pub use zip_loader::ZipLoader;

pub trait Loader {
    fn load(&self, name: &str) -> Result<String>;
}
