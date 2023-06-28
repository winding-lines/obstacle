use memmap2::{self, MmapAsRawDesc, MmapOptions};
use std::io::Result;
use std::ops::Deref;

/// Wrapped for the memmap2::Mmap.
pub struct Mmap(memmap2::Mmap);

impl Mmap {
    pub unsafe fn map<T: MmapAsRawDesc>(file: T) -> Result<Mmap> {
        Ok(Mmap(MmapOptions::new().map(file)?))
    }
}

impl AsRef<[u8]> for Mmap {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.deref()
    }
}
