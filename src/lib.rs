use memmap2;

/// Wrapped for the memmap2::Mmap.
pub struct Mmap {
    inner: memmap2::Mmap,
}
