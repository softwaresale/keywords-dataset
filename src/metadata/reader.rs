use std::io::{BufRead, BufReader, Read};
use crate::err::AppResult;
use crate::metadata::ArxivMetadata;

pub struct ArxivMetadataIter<T: Read> {
    reader: BufReader<T>,
}

impl<T: Read> ArxivMetadataIter<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    fn read_next(&mut self) -> AppResult<Option<ArxivMetadata>> {
        let ok_res = loop {
            let mut buffer = String::new();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => break None,
                Ok(_) => {},
                Err(err) => return Err(err.into()),
            }

            match serde_json::from_str(&buffer) {
                Ok(model) => break Some(model),
                Err(err) => return Err(err.into())
            }
        };

        Ok(ok_res)
    }
}

impl<T: Read> Iterator for ArxivMetadataIter<T> {
    type Item = ArxivMetadata;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_next() {
            Ok(None) | Err(_) => None,
            Ok(Some(value)) => Some(value)
        }
    }
}

