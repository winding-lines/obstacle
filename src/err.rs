
fn obsinate_err(msg: &str) -> Result<Mmap> {
    Err(Error::new(
        ErrorKind::InvalidData,
        msg,
    ))
}
