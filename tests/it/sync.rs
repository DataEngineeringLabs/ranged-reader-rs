use std::io::Read;

use range_reader::RangedReader;

fn test(calls: usize, call_size: usize, buffer: usize) {
    let length = 100;
    let range_fn = Box::new(move |start: usize, buf: &mut [u8]| {
        let iter = (start..start + buf.len()).map(|x| x as u8);
        buf.iter_mut().zip(iter).for_each(|(x, v)| *x = v);
        Ok(())
    });

    let mut reader = RangedReader::new(length, range_fn, vec![0; buffer]);

    let mut to = vec![0; call_size];
    let mut result = vec![];
    (0..calls).for_each(|i| {
        let _ = reader.read(&mut to);
        result.extend_from_slice(&to);
        assert_eq!(
            result,
            (0..(i + 1) * call_size)
                .map(|x| x as u8)
                .collect::<Vec<_>>()
        );
    });
}

#[test]
fn basics() {
    test(10, 5, 10);
    test(5, 20, 10);
    test(10, 7, 10);
}
