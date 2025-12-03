use crate::loaders::Loader;
use anyhow::Result;
use std::{fs, path::Path};

pub struct FileLoader {
    base: String,
}

impl FileLoader {
    pub fn new(base: &str) -> FileLoader {
        FileLoader {
            base: base.to_owned(),
        }
    }
}

impl Loader for FileLoader {
    fn load(&self, name: &str) -> Result<String> {
        let base = Path::new(&self.base);
        let file = base.join(name);
        let content = fs::read_to_string(file)?;
        Ok(content)
    }
}
