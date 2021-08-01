use std::io::{Read, Result, Seek, SeekFrom};

type FnAPI = Box<dyn Fn(usize, &mut [u8]) -> Result<()>>;

/// Implements `Read + Seek` for a (blocking) function that reads ranges of bytes.
/// # Implementation
/// This struct has an internal `Vec<u8>` that buffers calls.
pub struct RangedReader {
    pos: u64,
    length: u64,     // total size
    buffer: Vec<u8>, // a ring
    offset: usize,   // offset in the ring: buffer[:offset] have been read
    range_fn: FnAPI,
}

impl RangedReader {
    pub fn new(length: usize, range_fn: FnAPI, mut buffer: Vec<u8>) -> Self {
        let length = length as u64;
        buffer.clear();
        Self {
            pos: 0,
            range_fn,
            length,
            buffer,
            offset: 0,
        }
    }

    fn read_more(&mut self, to_consume: usize) -> Result<()> {
        let remaining = self.buffer.len() - self.offset;

        if to_consume < remaining {
            return Ok(());
        }
        let to_read = std::cmp::max(
            std::cmp::max(self.offset, to_consume),
            self.buffer.capacity(),
        ) - remaining;

        self.buffer.rotate_left(self.offset);
        self.buffer.resize(remaining + to_read, 0);

        (self.range_fn)(self.pos as usize, &mut self.buffer[remaining..])?;
        self.pos += to_read as u64;
        self.offset = 0;
        Ok(())
    }
}

impl Read for RangedReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let to_consume = buf.len();
        self.read_more(to_consume)?;

        // copy from the internal buffer.
        buf[..to_consume].copy_from_slice(&self.buffer[self.offset..self.offset + to_consume]);
        // and offset
        self.offset += to_consume;
        Ok(to_consume)
    }
}

impl Seek for RangedReader {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(pos) => self.pos = pos,
            SeekFrom::End(pos) => self.pos = (self.length as i64 + pos) as u64,
            SeekFrom::Current(pos) => self.pos = (self.pos as i64 + pos) as u64,
        };
        // todo: optimize: do not clear buffer and instead check whether we can just move the offset.
        self.offset = 0;
        self.buffer.clear();
        Ok(self.pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
