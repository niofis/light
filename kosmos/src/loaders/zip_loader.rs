use crate::loaders::Loader;
use anyhow::{Result, anyhow};
use rawzip::ZipSliceArchive;

pub struct ZipLoader {
    archive: ZipSliceArchive<Vec<u8>>,
}

impl ZipLoader {
    pub fn new(data: Vec<u8>) -> Result<ZipLoader> {
        let archive = rawzip::ZipArchive::from_slice(data)?;
        Ok(ZipLoader { archive })
    }
}

impl Loader for ZipLoader {
    fn load(&self, name: &str) -> Result<String> {
        let entry = self
            .archive
            .entries()
            .find(|p| {
                if let Ok(file_path) = p.as_ref()
                    && let Ok(normalized_file_path) = file_path.file_path().try_normalize()
                {
                    return normalized_file_path.as_ref() == name;
                }
                false
            })
            .ok_or(anyhow!("file not found: {}", name))??;
        let wayfinder = entry.wayfinder();
        let local_entry = self.archive.get_entry(wayfinder)?;
        let mut actual = Vec::new();
        let decompressor = flate2::bufread::DeflateDecoder::new(local_entry.data());
        let mut reader = local_entry.verifying_reader(decompressor);
        std::io::copy(&mut reader, &mut actual)?;
        let content = str::from_utf8(&actual)?.to_string();
        Ok(content)
    }
}
