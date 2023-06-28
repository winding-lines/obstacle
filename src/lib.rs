use memmap2::{self, MmapAsRawDesc, MmapOptions};
use std::fmt::Debug;
use std::io::Result;
use std::ops::Deref;

mod cloud;
mod err;

/// Wrapped for the memmap2::Mmap.
pub struct Mmap(memmap2::Mmap);

impl Mmap {

    /// Create a memory map from an object with the MmapAsRawDesc trait.
    pub unsafe fn map<T: MmapAsRawDesc + Debug>(file: T) -> Result<Mmap> {
        println!("\n\n{:?}\n\n", file);
        Ok(Mmap(MmapOptions::new().map(file)?))
    }

    /// Create a memory map from a file or url path.
    pub from_url(url: &str) -> Result<Mmap> {
        let file = File::open(url)?;
        unsafe { Mmap::map(file) }
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
