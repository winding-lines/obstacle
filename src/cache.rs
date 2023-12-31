//! Cache files locally in order to provide a mmap interface and provide faster access.
//!
//! When saving a file locally we create a directory structure that mirrors the cloud under ~/.cache/obstinate.
//! Each url becomes a folder and the content of the file is saved with a name based on the e-tag of the file.

use crate::err::{obstinate_err, ObstacleError};
use crate::glob::CloudLocation;
use crate::{build, get_cloud_options};
use futures_util::StreamExt;
use home::home_dir;
use log::debug;
use object_store::path::Path as ObjectStorePath;
use object_store::{GetOptions, ObjectStore};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{self, PathBuf};
use tokio::fs::{read_dir, remove_file, rename};
use uuid::Uuid;

/// Build a local file for caching a given url.
/// We use the full url, including the file name, as the directory name.
/// This allows multiple versions of the same file to be cached.
fn _local_path_for_cloud_location(location: &CloudLocation) -> Result<PathBuf, ObstacleError> {
    let mut base = home_dir().unwrap();
    base.push(".cache/obstinate");
    create_dir_all(&base)?;
    base.push(&location.scheme);
    base.push(&location.bucket);
    base.push(location.prefix.trim_start_matches("/"));
    if !base.try_exists()? {
        debug!("creating directory {}", base.display());
        create_dir_all(&base)?;
    }
    Ok(base)
}

/// Delete any other content_* files that do not match the active content.
async fn _cleanup_content(local_path: &PathBuf, active_content: &str) -> Result<(), ObstacleError> {
    debug!("cleaning up {}", local_path.display());
    let mut dir = read_dir(&local_path).await?;
    loop {
        let entry = dir.next_entry().await?;
        match entry {
            None => break,
            Some(entry) => {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                if file_name_str == active_content {
                    continue;
                }
                if !file_name_str.starts_with("content_") {
                    continue;
                }
                debug!("removing {}", file_name_str);
                remove_file(entry.path()).await?;
            }
        }
    }
    Ok(())
}

enum DownloadResult {
    /// The file was downloaded and saved locally.
    Downloaded(File),
    /// The file was already downloaded and is available locally.
    Cached(File),
    /// The file changed in the cloud during the download process.
    Retry,
    /// The file was not found.
    NotFound,
}

async fn _download_one(
    cloud_location: &CloudLocation,
    object_store: &Box<dyn ObjectStore>,
) -> Result<DownloadResult, ObstacleError> {
    let os_path: ObjectStorePath = ObjectStorePath::from_url_path(&cloud_location.prefix)?;

    // Get the active e-tag for the object in the cloud, this cannot be read as part of the `get*` call.
    // https://github.com/apache/arrow-rs/discussions/4495

    debug!("getting metadata for {}", os_path);
    let cloud_metadata = object_store.head(&os_path).await?;
    let desired_filename = format!(
        "content_{}",
        cloud_metadata.e_tag.clone().unwrap_or("default".into())
    );
    debug!("desired filename {}", desired_filename);

    // check if we have a local copy of the file and return it if we do.
    let local_base = _local_path_for_cloud_location(&cloud_location)?;
    let local_path = local_base.join(path::Path::new(&desired_filename));
    if local_path.exists() {
        debug!("returning existing file {}", local_path.display());
        return Ok(DownloadResult::Cached(File::open(local_path)?));
    }

    debug!("about to cleanup");

    // Delete any old content_* files and download the latest version.
    _cleanup_content(&local_base, &desired_filename).await?;

    debug!("About to download");
    // Download the file if it matches the e-tag.
    let get_options = GetOptions {
        if_match: cloud_metadata.e_tag,
        ..GetOptions::default()
    };
    let get_result = object_store.get_opts(&os_path, get_options).await;

    match get_result {
        Err(ref err) => match err {
            // The object has changed in the cloud, loop.
            object_store::Error::Precondition { .. } => {
                debug!("object changed in the cloud, retrying");
                return Ok(DownloadResult::Retry);
            }
            object_store::Error::NotFound { .. } => {
                // The object does not exist in the cloud, return None.
                debug!("object not found in the cloud");
                return Ok(DownloadResult::NotFound);
            }
            _ => return Err(ObstacleError::new(err.to_string())),
        },
        Ok(result) => {
            let mut stream = result.into_stream();

            // Create a temporary file and download the object to it.
            let tempfile = local_base.join(path::Path::new(&format!(
                "temp_{}",
                Uuid::new_v4().to_string()
            )));
            {
                debug!("Downloading to temporary file {}.", tempfile.display());
                let mut local_file = File::create(&tempfile)?;
                while let Some(buffer) = stream.next().await {
                    let bytes = buffer.unwrap();
                    local_file.write_all(&bytes)?;
                }
                local_file.sync_all()?;
            }
            // Now rename the successful download to the desired filename.
            rename(&tempfile, &local_path).await?;

            // Return the cached file.
            let file = File::open(local_path).unwrap();
            return Ok(DownloadResult::Downloaded(file));
        }
    }
}

/// Download a file from the cloud and cache it locally.
///
/// Because of current limits in the API we need to use head() to get the e-tag and then download with get_opts().
/// This is not ideal because:
/// 1. we need to make two calls to the cloud to download a file.
/// 2. there is a race condition where an object could change between the head() and get_opts() calls.
///
pub async fn download_file(url: &str) -> Result<Option<File>, ObstacleError> {
    let cloud_options = get_cloud_options();

    let (cloud_location, object_store) = build(url, cloud_options)?;
    for _attempt in 0..10 {
        debug!("attempt {} at downloading {}", _attempt, url);
        match _download_one(&cloud_location, &object_store).await {
            Ok(DownloadResult::Downloaded(file)) => return Ok(Some(file)),
            Ok(DownloadResult::Cached(file)) => return Ok(Some(file)),
            Ok(DownloadResult::Retry) => continue,
            Ok(DownloadResult::NotFound) => return Ok(None),
            Err(err) => return Err(err),
        }
    }
    return obstinate_err("Failed to download file after 10 attempts");
}
