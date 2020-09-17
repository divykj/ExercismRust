use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    reads: Vec<usize>,
}

impl<R: Read> ReadStats<R> {
    pub fn new(reader: R) -> ReadStats<R> {
        Self {
            reader,
            reads: vec![],
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.reads.iter().fold(0 as usize, |acc, x| acc + x)
    }

    pub fn reads(&self) -> usize {
        self.reads.len()
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.reader.read(buf)?;
        self.reads.push(bytes);
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    writer: W,
    writes: Vec<usize>,
}

impl<W: Write> WriteStats<W> {
    pub fn new(writer: W) -> WriteStats<W> {
        Self {
            writer,
            writes: vec![],
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.writes.iter().fold(0 as usize, |acc, x| acc + x)
    }

    pub fn writes(&self) -> usize {
        self.writes.len()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.writer.write(buf)?;
        self.writes.push(bytes);
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
