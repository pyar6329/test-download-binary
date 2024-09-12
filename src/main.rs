use anyhow::{Error, Result};
use rusoto_core::Region::UsEast1;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::env;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bucket_name = env::var("S3_BUCKET_NAME").unwrap_or("foobar".to_string());

    let download_url = Url::parse("https://www.rust-lang.org/logos/rust-logo-512x512.png")?;
    let image_bytes = download_file(&download_url).await?;
    let bucket_path = "additional/rust-logo.png";

    send_to_s3(&image_bytes, &bucket_name, bucket_path).await?;

    Ok(())
}

async fn download_file(url: &Url) -> Result<Vec<u8>, Error> {
    let url_string: String = url.to_string();
    let response = reqwest::get(url_string).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

async fn send_to_s3(bytes: &[u8], bucket: &str, key: &str) -> Result<(), Error> {
    let client = S3Client::new(UsEast1);
    let put_request = PutObjectRequest {
        bucket: bucket.to_string(),
        key: key.to_string(),
        body: Some(bytes.to_vec().into()),
        ..Default::default()
    };
    client.put_object(put_request).await?;
    Ok(())
}
