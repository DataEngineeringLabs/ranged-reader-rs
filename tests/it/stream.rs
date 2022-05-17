use std::io::SeekFrom;

use futures::{future::BoxFuture, AsyncReadExt, AsyncSeekExt};

use range_reader::{RangeOutput, RangedAsyncReader};

// range that returns a vector of `(x % 255) for x in start..start+length`
fn get_mock_range(min_size: usize) -> RangedAsyncReader {
    let ranged_future = Box::new(move |start: u64, length: usize| {
        Box::pin(async move {
            let data = (start..start + length as u64)
                .map(|x| (x % 255) as u8)
                .collect();
            Ok(RangeOutput { start, data })
        }) as BoxFuture<'static, std::io::Result<RangeOutput>>
    });

    let length = 100;
    RangedAsyncReader::new(length, min_size, ranged_future)
}

// performs `calls` calls of length `call_size` and checks that they return the expected result.
async fn test(calls: usize, call_size: usize, min_size: usize) {
    let mut reader = get_mock_range(min_size);

    let mut to = vec![0; call_size];
    let mut result = vec![];
    for i in 0..calls {
        let _ = reader.read(&mut to).await;
        result.extend_from_slice(&to);
        assert_eq!(
            result,
            (0..(i + 1) * call_size)
                .map(|x| x as u8)
                .collect::<Vec<_>>()
        );
    }
}

#[tokio::test]
async fn basics() {
    test(10, 5, 10).await;
    test(5, 20, 10).await;
    test(10, 7, 10).await;
}

// tests that multiple `seek`s followed by `read` are correct
#[tokio::test]
async fn seek_inside() -> std::io::Result<()> {
    let mut reader = get_mock_range(100);

    for seek in 0u8..99 {
        let length = 100 - seek;
        let mut result = vec![0; length as usize];
        reader.seek(SeekFrom::Start(seek as u64)).await?;
        reader.read_exact(&mut result).await?;
        assert_eq!(result, (seek..100).collect::<Vec<_>>());
    }

    Ok(())
}

// tests that `seek` followed by `read` is correct
#[tokio::test]
async fn seek_new() -> std::io::Result<()> {
    for seek in 0u8..99 {
        let mut reader = get_mock_range(100);
        let length = 100 - seek;
        let mut result = vec![0; length as usize];
        reader.seek(SeekFrom::Start(seek as u64)).await?;
        reader.read_exact(&mut result).await?;
        assert_eq!(result, (seek..100).collect::<Vec<_>>());
    }

    Ok(())
}

// tests that reading incomplete overlapping regions work as expected
#[tokio::test]
async fn seek_split() -> std::io::Result<()> {
    let mut reader = get_mock_range(2);
    let length = 20;
    let mut result = vec![0; length as usize];
    reader.read_exact(&mut result).await?;
    assert_eq!(result, (0..length).collect::<Vec<_>>());

    reader.seek(SeekFrom::Start(10)).await?;
    let mut result = vec![0; length as usize];
    reader.read_exact(&mut result).await?;
    assert_eq!(result, (10..10 + length).collect::<Vec<_>>());

    Ok(())
}
