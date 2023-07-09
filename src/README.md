Utility classes to integrate the (Object Store)[https://crates.io/crates/object_store] in an application. This started initially as an effort in (Polars)[https://pola.rs]

BRAINSTORM

Since the Polars implementation relies heavily on (mmap)[https://crates.io/crates/memmap2] our API will be inspired from that crate.

1. Use the mmap interface for the advise operation.
2. Change the polars code to issue `advise()` before access.
3. Implement the advise on top of `object_store` in the current crate.


DELIVERY PHASES

1. download
2. cache 
3. validated the cache
4. sparse downloads 