//! This example assumes that you have installed minio locally.
//! Run with:
//!     cargo run --example   minio --features aws
use awscreds::Credentials;
use object_store::ClientConfigKey;
use obstinate::AmazonS3ConfigKey as Key;
use obstinate::{self, set_cloud_options};
use std::str::from_utf8;

pub fn main() {
    let cred = Credentials::default().unwrap();

    // Propagate the credentials and other cloud options.
    let cloud_options = obstinate::CloudOptions::default().with_aws([
        (Key::AccessKeyId, &cred.access_key.unwrap()),
        (Key::SecretAccessKey, &cred.secret_key.unwrap()),
        (Key::Region, &"us-east-1a".into()),
        (Key::Client(ClientConfigKey::AllowHttp), &"true".into()),
        (Key::Endpoint, &"http://localhost:9000".into()),
    ]);
    set_cloud_options(cloud_options);
    let mmaped = obstinate::Mmap::from_url(&"s3://one/foods2.csv").unwrap();
    print!("content: {}.", from_utf8(&mmaped[..]).unwrap());
}
