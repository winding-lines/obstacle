use crate::err::ObstinateError;
use crate::get_cloud_options;
use futures_util::StreamExt;
use object_store::path::Path;
use object_store::ObjectStore;
use std::fs::File;
use std::io::Write;

pub async fn download_file(_url: &str) -> Result<File, ObstinateError> {
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
    return Ok(file);
}
