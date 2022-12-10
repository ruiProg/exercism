use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    source: R,
    reads_count: usize,
    bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(source: R) -> ReadStats<R> {
        Self {
            source,
            reads_count: 0,
            bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.source
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads_count += 1;
        self.source.read(buf).map(|bytes_read| {
            self.bytes_read += bytes_read;
            bytes_read
        })
    }
}

pub struct WriteStats<W> {
    sink: W,
    writes_count: usize,
    bytes_written: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(sink: W) -> WriteStats<W> {
        Self {
            sink,
            writes_count: 0,
            bytes_written: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.sink
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.writes_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes_count += 1;
        self.sink.write(buf).map(|bytes_written| {
            self.bytes_written += bytes_written;
            bytes_written
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.sink.flush()
    }
}
