use std::io::{BufWriter, Write};
use keyword_dataset_rs::err::AppResult;
use keyword_dataset_rs::training::TrainingRecord;

pub trait OutputFormatter: Write {
    fn write_record(&mut self, record: TrainingRecord) -> AppResult<()>;
}

pub struct NdJsonOutputFormatter<OutputStream: Write> {
    writer: BufWriter<OutputStream>,
}

impl<OutputStream: 'static + Write> NdJsonOutputFormatter<OutputStream> {
    pub fn new(writer: OutputStream) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }
    
    pub fn into_boxed_trait(self) -> Box<dyn OutputFormatter> {
        Box::new(self) as Box<dyn OutputFormatter>
    }
}

impl<OutputStream: Write> Write for NdJsonOutputFormatter<OutputStream> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<OutputStream: Write> OutputFormatter for NdJsonOutputFormatter<OutputStream> {
    fn write_record(&mut self, record: TrainingRecord) -> AppResult<()> {
        const NEWLINE: [u8; 1] = ['\n' as u8];
        let bytes = serde_json::to_vec(&record)?;
        self.write_all(&bytes)?;
        self.write(&NEWLINE)?;
        Ok(())
    }
}
