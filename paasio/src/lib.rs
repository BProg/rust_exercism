use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    bytes_through: usize,
    reads: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            data: wrapped,
            bytes_through: 0,
            reads: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        let read_count = self.data.read(buf)?;
        self.bytes_through += read_count;
        Ok(read_count)
    }
}

pub struct WriteStats<W> {
    data: W,
    writes: usize,
    bytes_through: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            data: wrapped,
            bytes_through: 0,
            writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        let write_count = self.data.write(buf)?;
        self.bytes_through += write_count;
        Ok(write_count)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
