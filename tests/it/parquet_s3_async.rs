use std::io::Result;
use std::sync::Arc;

use futures::pin_mut;
use futures::{future::BoxFuture, StreamExt};
use s3::Bucket;

use parquet2::read::{get_page_stream, read_metadata_async};
use range_reader::{RangeOutput, RangedAsyncReader};

#[tokio::test]
async fn main() -> Result<()> {
    let bucket_name = "dev-jorgecardleitao";
    let region = "eu-central-1".parse().unwrap();
    let bucket = Bucket::new_public(bucket_name, region).unwrap();
    let path = "benches_65536.parquet".to_string();

    let (data, _) = bucket.head_object(&path).await.unwrap();
    let length = data.content_length.unwrap() as usize;
    println!("total size in bytes: {}", length);

    let range_get = Box::new(move |start: u64, length: usize| {
        let bucket = bucket.clone();
        let path = path.clone();
        Box::pin(async move {
            let bucket = bucket.clone();
            let path = path.clone();
            // to get a sense of what is being queried in s3
            let (mut data, _) = bucket
                // -1 because ranges are inclusive in `get_object_range`
                .get_object_range(&path, start, Some(start + length as u64 - 1))
                .await
                .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.to_string()))?;
            data.truncate(length);
            Ok(RangeOutput { start, data })
        }) as BoxFuture<'static, std::io::Result<RangeOutput>>
    });

    // at least 4kb per s3 request. Adjust as you like.
    let mut reader = RangedAsyncReader::new(length, 4 * 1024, range_get);

    let metadata = read_metadata_async(&mut reader)
        .await
        .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.to_string()))?;

    assert_eq!(524288, metadata.num_rows);

    // pages of the first row group and first column
    let column_metadata = &metadata.row_groups[0].columns()[0];
    let pages = get_page_stream(column_metadata, &mut reader, vec![], Arc::new(|_, _| true))
        .await
        .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.to_string()))?;

    pin_mut!(pages);
    while let Some(maybe_page) = pages.next().await {
        let page = maybe_page
            .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.to_string()))?;
        assert_eq!(page.num_values(), 524288);
    }
    Ok(())
}
