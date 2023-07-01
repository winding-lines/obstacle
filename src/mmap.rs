use crate::cloud::CloudType;
use crate::get_cloud_options;
use futures_util::StreamExt;
use memmap2::{self, MmapAsRawDesc, MmapOptions};
use object_store::path::Path;
use object_store::ObjectStore;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::Result;
use std::io::Write;
use std::ops::Deref;
use std::str::FromStr;
use tokio;

/// Wrapped for the memmap2::Mmap.
pub struct Mmap(memmap2::Mmap);

impl Mmap {
    /// Create a memory map from an object with the MmapAsRawDesc trait.
    pub unsafe fn map<T: MmapAsRawDesc + Debug>(file: T) -> Result<Mmap> {
        Ok(Mmap(MmapOptions::new().map(file)?))
    }

    /// Create a memory map from a file or url path.
    #[tokio::main]
    pub async fn from_url(url: &str) -> Result<Mmap> {
        match CloudType::from_str(url) {
            Ok(cloud_type) => {
                let cloud_options = get_cloud_options().unwrap();
                let object_store = cloud_options.build_aws("one").unwrap();
                let os_path = Path::from("foods2.csv");
                let mut stream = object_store.get(&os_path).await.unwrap().into_stream();
                let local_path = "local_foods2.csv";
                let mut local_file = File::create(local_path).unwrap();
                while let Some(buffer) = stream.next().await {
                    let bytes = buffer.unwrap();
                    local_file.write_all(&bytes).unwrap();
                }
                local_file.flush().unwrap();
                let file = File::open(local_path).unwrap();
                unsafe { Mmap::map(&file) }
            }
            Err(_) => {
                let file = File::open(url)?;
                unsafe { Mmap::map(&file) }
            }
        }
    }
}

impl Deref for Mmap {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.0.deref()
    }
}

impl AsRef<[u8]> for Mmap {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.deref()
    }
}
