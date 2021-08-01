use parquet2::read::read_metadata;
use ranged_reader::RangedReader;
use s3::Bucket;

fn main() {
    let bucket_name = "ursa-labs-taxi-data";
    let region = "us-east-2".parse().unwrap();
    let bucket = Bucket::new_public(bucket_name, region).unwrap();
    let path = "2009/01/data.parquet".to_string();

    let (data, _) = bucket.head_object_blocking(&path).unwrap();
    let length = data.content_length.unwrap() as usize;

    let moved_path = path.clone();
    let range_fn = Box::new(move |start: usize, buf: &mut [u8]| {
        let (mut data, _) = bucket
            .get_object_range_blocking(
                &moved_path,
                start as u64,
                Some(start as u64 + buf.len() as u64),
            )
            .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.to_string()))?;
        data.truncate(buf.len());
        buf[..data.len()].copy_from_slice(&data);
        Ok(())
    });

    let buffer = 1024;

    let mut reader = RangedReader::new(length, range_fn, vec![0; buffer]);

    let metadata = read_metadata(&mut reader).unwrap();

    let num_rows: usize = metadata
        .row_groups
        .iter()
        .map(|group| group.num_rows() as usize)
        .sum();
    println!(
        "The file \"{}/{}\" has a total of {} row groups with a total of {} rows",
        bucket_name,
        path,
        metadata.row_groups.len(),
        num_rows
    );
}
