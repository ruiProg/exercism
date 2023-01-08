use std::{cmp::Ordering, mem::swap};

pub struct CircularBuffer<T> {
    data: Vec<T>,
    read_index: usize,
    write_index: usize,
    size: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Default,
{
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::new();
        data.resize_with(capacity, T::default);

        Self {
            data,
            read_index: 0,
            write_index: 0,
            size: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        match self.size.cmp(&self.data.len()) {
            Ordering::Equal => Err(Error::FullBuffer),
            _ => {
                self.write_impl(element);
                Ok(())
            }
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.size {
            0 => Err(Error::EmptyBuffer),
            _ => {
                let mut result = T::default();
                swap(&mut result, &mut self.data[self.read_index]);
                self.read_index = (self.read_index + 1) % self.data.len();
                self.size -= 1;
                Ok(result)
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new(self.data.len());
    }

    pub fn overwrite(&mut self, element: T) {
        if self.size == self.data.len() {
            self.read_index += 1;
        }
        self.write_impl(element);
    }

    fn write_impl(&mut self, element: T) {
        self.data[self.write_index] = element;
        self.write_index = (self.write_index + 1) % self.data.len();
        self.size += 1;
    }
}
