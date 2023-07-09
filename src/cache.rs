use crate::err::ObstinateError;
use crate::glob::CloudLocation;
use crate::{build, get_cloud_options};
use futures_util::StreamExt;
use home::home_dir;
use object_store::path::Path as ObjectStorePath;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use url::Url;

/// Build a local file for caching a given url.
fn _local_path_for_url(location: &CloudLocation) -> Result<PathBuf, ObstinateError> {
    let mut base = home_dir().unwrap();
    base.push(".cache/obstinate");
    create_dir_all(&base)?;
    base.push(&location.scheme);
    base.push(&location.bucket);
    base.push(location.prefix.trim_start_matches("/"));
    println!("local path: {:?}", base);
    base.parent().map_or(Ok(()), |p| create_dir_all(p))?;
    Ok(base)
}

pub async fn download_file(url: &str) -> Result<File, ObstinateError> {
    let parsed = Url::parse(url).map_err(ObstinateError::from_err)?;
    let cloud_options = get_cloud_options();

    let (cloud_location, object_store) = build(url, cloud_options)?;
    let os_path: ObjectStorePath = ObjectStorePath::from(parsed.path());
    let mut stream = object_store.get(&os_path).await.unwrap().into_stream();

    let local_path = _local_path_for_url(&cloud_location)?;
    if local_path.exists() {
        return Ok(File::open(local_path)?);
    }
    let mut local_file = File::create(&local_path)?;
    while let Some(buffer) = stream.next().await {
        let bytes = buffer.unwrap();
        local_file.write_all(&bytes).unwrap();
    }
    local_file.flush().unwrap();
    let file = File::open(local_path).unwrap();
    return Ok(file);
}
