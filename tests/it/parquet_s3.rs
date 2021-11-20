use parquet2::read::read_metadata;
use range_reader::RangedReader;
use s3::Bucket;

#[test]
fn main() {
    let bucket_name = "dev-jorgecardleitao";
    let region = "eu-central-1".parse().unwrap();
    let bucket = Bucket::new_public(bucket_name, region).unwrap();
    let path = "benches_65536.parquet".to_string();

    let (data, _) = bucket.head_object_blocking(&path).unwrap();
    let length = data.content_length.unwrap() as usize;

    let range_fn = Box::new(move |start: usize, buf: &mut [u8]| {
        let (mut data, _) = bucket
            // -1 because ranges are inclusive in `get_object_range`
            .get_object_range_blocking(
                &path,
                start as u64,
                Some(start as u64 + buf.len() as u64 - 1),
            )
            .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.to_string()))?;
        data.truncate(buf.len());
        buf[..data.len()].copy_from_slice(&data);
        Ok(())
    });

    let buffer = 1024 * 4; // 4 kb per request.

    let mut reader = RangedReader::new(length, range_fn, vec![0; buffer]);

    let metadata = read_metadata(&mut reader).unwrap();

    let num_rows: usize = metadata
        .row_groups
        .iter()
        .map(|group| group.num_rows() as usize)
        .sum();
    assert_eq!(num_rows, 524288);
    assert_eq!(metadata.row_groups.len(), 1);
}
