use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes_total: usize,     // The total number of bytes read/written.
    operation_count: usize, // The total number of read/write operations.
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            bytes_total: 0,
            operation_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_total
    }

    pub fn reads(&self) -> usize {
        self.operation_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.operation_count += 1;
        let bytes = self.wrapped.read(buf)?;
        self.bytes_total += bytes;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_total: usize,     // The total number of bytes read/written.
    operation_count: usize, // The total number of read/write operations.
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            bytes_total: 0,
            operation_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_total
    }

    pub fn writes(&self) -> usize {
        self.operation_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.operation_count += 1;
        let bytes = self.wrapped.write(buf)?;
        self.bytes_total += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
