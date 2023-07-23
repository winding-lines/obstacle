#[cfg(feature = "async")]
use crate::cache::download_file;
use crate::cloud::CloudType;
use crate::err::ObstacleError;
use memmap2::{self, MmapAsRawDesc, MmapOptions};
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::ops::Deref;
use std::str::FromStr;
use tokio;

/// Wrapped for the memmap2::Mmap.
pub struct Mmap(memmap2::Mmap);

async fn _open_url(url: &str) -> Result<Option<File>, ObstacleError> {
    match CloudType::from_str(url) {
        Ok(_cloud_type) => {
            #[cfg(feature = "async")]
            {
                Ok(download_file(url).await?)
            }
            #[cfg(not(feature = "async"))]
            {
                panic!("at least one of the cloud features must be enabled")
            }
        }
        Err(_) => {
            // Check to see if the file exists locally.
            let file = File::open(url)?;
            Ok(Some(file))
        }
    }
}

#[tokio::main]
/// Create a file map from a local or cloud path.
pub async fn open_url<S: AsRef<str>>(url: S) -> Result<Option<File>, ObstacleError> {
    _open_url(url.as_ref()).await
}

impl Mmap {
    /// Create a memory map from an object with the MmapAsRawDesc trait.
    pub unsafe fn map<T: MmapAsRawDesc + Debug>(file: T) -> Result<Mmap, io::Error> {
        Ok(Mmap(MmapOptions::new().map(file)?))
    }

    #[tokio::main]
    pub async fn from_url(url: &str) -> Result<Option<Mmap>, ObstacleError> {
        _open_url(url)
            .await?
            .as_ref()
            .map(|file| unsafe { Ok(Self::map(file)?) })
            .transpose()
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
