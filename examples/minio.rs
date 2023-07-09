//! Download a file from minio (S3 API compatible) and print its content.
//!
//! This example assumes that:
//! 1. you have installed minio locally.
//! 2. you have created a bucket named "one" in minio.
//! 3. you have uploaded a file named "foods2.csv" to the bucket.
//! 4. you have setup the access key and secret key for minio and saved it under ~/.aws/credentials.
//!
//! Run with:
//!     cargo run --example   minio --features aws

#[cfg(feature = "aws")]
fn mmap_from_url() {
    use awscreds::Credentials;
    use object_store::ClientConfigKey;
    use obstacle::set_cloud_options;
    use obstacle::AmazonS3ConfigKey as Key;
    use std::str::from_utf8;

    let cred = Credentials::default().unwrap();

    // Propagate the credentials and other cloud options.
    let cloud_options = obstacle::CloudOptions::default().with_aws([
        (Key::AccessKeyId, &cred.access_key.unwrap()),
        (Key::SecretAccessKey, &cred.secret_key.unwrap()),
        (Key::Region, &"us-east-1a".into()),
        (Key::Client(ClientConfigKey::AllowHttp), &"true".into()),
        (Key::Endpoint, &"http://localhost:9000".into()),
    ]);
    set_cloud_options(cloud_options);
    let mmaped = obstacle::Mmap::from_url(&"s3://one/foods2.csv").unwrap();
    print!("content: {}.", from_utf8(&mmaped[..]).unwrap());
}

pub fn main() {
    #[cfg(feature = "aws")]
    {
        mmap_from_url();
    }
    #[cfg(not(feature = "aws"))]
    {
        println!("Please enable the aws feature to run this example.");
    }
}
